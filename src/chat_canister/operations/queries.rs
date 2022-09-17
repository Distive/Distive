use std::ops::Add;

use crate::{
    shared::types::{ExportChunk, ExportParam, GetThreadParam, IPage},
    CHANNELS, TIME_CREATED,
};
use chat_engine::{context::Context, Channel};
use csv::Writer;
use ic_cdk::{
    api::canister_balance,
    export::candid::{CandidType, Deserialize},
};
use ic_cdk_macros::query;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Status {
    remaining_cycles: u64,
    time_created: u64,
    is_empty: bool,
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
pub fn status() -> Status {
    Status {
        remaining_cycles: canister_balance(),
        time_created: TIME_CREATED.with(|time_created| time_created.clone()),
        is_empty: CHANNELS.with(|channels| channels.borrow().is_empty()),
    }
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_thread(param: GetThreadParam) -> IPage {
    let context = Context::new(param.metadata_user_ids.clone().unwrap_or_default());

    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let channel = channels
            .entry(param.channel_id.to_string())
            .or_insert_with(|| Channel::new(param.channel_id.to_string()));
        let page = channel.get_page(
            &(param.limit as usize),
            param.cursor.as_ref(),
            Some(context),
        );
        page.map(|p| p.into()).unwrap_or_default()
    })
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn export_comments(params: ExportParam) -> ExportChunk {
    const MAX_CAPACITY: usize = 1_800_000;

    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let channels_iterator = channels.iter();
        let flattened_channels_comments_iterator =
            channels_iterator.flat_map(|(_, channel)| channel.export());
        let (data, last_cursor) = flattened_channels_comments_iterator
            .skip(params.cursor.into())
            .fold_while(
                (Vec::with_capacity(MAX_CAPACITY), params.cursor),
                |(acc, cursor), comment| {
                    if acc.len() > MAX_CAPACITY {
                        Done((acc, cursor))
                    } else {
                        let mut csv_writer = Writer::from_writer(acc);
                        csv_writer.serialize(comment);
                        let new_acc = csv_writer.into_inner().expect("Failed to write CSV");
                        Continue((new_acc, cursor.add(1)))
                    }
                },
            )
            .into_inner();

        ExportChunk {
            data,
            next_cursor: (last_cursor != params.cursor).then(|| last_cursor),
        }
    })
}
