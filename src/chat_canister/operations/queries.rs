use crate::{
    shared::types::{ExportParam, GetThreadParam, IPage},
    CHANNELS, TIME_CREATED,
};
use chat_engine::{context::Context, Channel};
use ic_cdk::{
    api::canister_balance,
    export::candid::{CandidType, Deserialize},
};
use ic_cdk_macros::query;
use sha2::{Digest, Sha256};

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

 
// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
fn export_comments(params: ExportParam) {
    let mut current_cursor: Option<String> = None;
    let mut hasher = Sha256::new();
    let ExportParam { cursor } = params;

    let channels = CHANNELS.with(|channels| channels);
    let exported_data = channels.borrow().iter();

    // exported_data skip till cursor -> write to 

}
