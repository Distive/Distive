use chat_engine::{context::Context, Channel};
use ic_cdk::{
    api::canister_balance,
    export::candid::{CandidType, Deserialize},
};
use ic_cdk_macros::query;

use crate::{shared::types::{GetThreadParam, IPage}, CHANNELS};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Status {
    remaining_cycles: u64,
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
pub fn status() -> Status {
    Status {
        remaining_cycles: canister_balance(),
    }
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_thread(param: GetThreadParam) -> IPage {
    let user_id = ic_cdk::caller().to_string();
    let context = Context::new(user_id);

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
