mod operations;
mod shared;

use chat_engine::Channel;
use ic_cdk::api::time;
use operations::{queries::*, updates::*};
use shared::types::{DeleteCommentParam, GetThreadParam, IPage, UpsertCommentParam};
use indexmap::IndexMap;
use std::cell::RefCell;

thread_local! {
  pub static CHANNELS: RefCell<IndexMap<String, Channel>> = RefCell::new(IndexMap::new());
  pub static TIME_CREATED: u64 = time();
}

ic_cdk::export::candid::export_service!();
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
