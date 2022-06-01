use crate::{CHANNELS, shared::{functions::authenticate_user_and_comment_action, types::DeleteCommentParam}};

use chat_engine::{
    context::Context,
    metadata::{MetadataInput}
};

use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};
use ic_cdk_macros::{update};


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ToggleMetadataParam {
    channel_id: String,
    comment_id: String,
    label: String,
}

#[update]
#[ic_cdk::export::candid::candid_method(update)]
pub fn toggle_metadata(param: ToggleMetadataParam) -> bool {
    let user_id = ic_cdk::caller();
    if Principal::anonymous().eq(&user_id) {
        false
    } else {
        let ToggleMetadataParam {
            channel_id,
            comment_id,
            label,
        } = param;

        CHANNELS.with(|channels| {
            let mut channels = channels.borrow_mut();
            channels
                .entry(channel_id.to_string())
                .and_modify(|channel| {
                    channel.toggle_comment_metadata(
                        &comment_id,
                        MetadataInput {
                            label: label.to_string(),
                            user_id: user_id.to_string(),
                        },
                    )
                });
        });

        true
    }
}


#[update]
#[ic_cdk::export::candid::candid_method(update)]
pub fn delete_comment(param: DeleteCommentParam) -> String {
    let user_id = ic_cdk::caller().to_string();
    let context = Context::new(user_id);
  
    let _result = authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        Some(context),
        |channel| {
            channel.delete_comment(param.comment_id.clone());
        },
    );

    "".to_string()
}
