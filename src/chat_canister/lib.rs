mod operations;
mod shared;

use chat_engine::Channel;
use hashbrown::HashMap;
use operations::{queries::*, updates::*};
use shared::types::{DeleteCommentParam, GetThreadParam, IPage, UpsertCommentParam};

use std::cell::RefCell;

thread_local! {
  pub static CHANNELS: RefCell<HashMap<String, Channel>> = RefCell::new(HashMap::new());
}

ic_cdk::export::candid::export_service!();
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
