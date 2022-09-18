use crate::{
    shared::{
        constants::TREASURY_CANISTER_ID,
        functions::authenticate_user_and_comment_action,
        types::{DeleteCommentParam, UpsertCommentParam},
    },
    CHANNELS,
};
use chat_engine::{
    comment::{CommentExport, CommentInput},
    metadata::MetadataInput,
    Channel,
};
use csv::{Reader, ReaderBuilder};
use ic_cdk::{
    api::time,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};
use ic_cdk_macros::{init, update};

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
    let _result = authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        None,
        |channel| {
            channel.delete_comment(param.comment_id.clone());
        },
    );

    "".to_string()
}

#[update]
#[ic_cdk::export::candid::candid_method(update)]
pub async fn wallet_receive() -> () {
    let treasury_canister = Principal::from_text(TREASURY_CANISTER_ID).unwrap();
    let amount = ic_cdk::api::call::msg_cycles_available();

    if amount > 0 {
        let (taxed_cycles, canister_cycles): (u64, u64) = {
            // 5% of the amount is sent to the treasury canister
            let taxed_cycles = (amount as f64 * 0.05) as u64;
            let canister_cycles = amount - taxed_cycles;
            (taxed_cycles, canister_cycles)
        };

        ic_cdk::api::call::msg_cycles_accept(canister_cycles);

        let _ = ic_cdk::api::call::call_with_payment::<(), ()>(
            treasury_canister,
            "wallet_receive",
            (),
            taxed_cycles,
        )
        .await;
    }
}

#[update]
#[ic_cdk::export::candid::candid_method(update)]
fn upsert_comment(param: UpsertCommentParam) -> String {
    let caller = ic_cdk::caller();

    match authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        None,
        |channel| {
            let comment_input = CommentInput {
                content: param.message.to_string(),
                id: param.comment_id.clone(),
                parent_id: param.parent_id.clone(),
                user_id: caller.to_string(),
                created_at: time(),
                channel_id: param.channel_id.clone(),
            };
            channel.upsert_comment(comment_input, None)
        },
    ) {
        Ok(result) => match result {
            Ok(output) => output.id,
            Err(message) => message,
        },
        Err(message) => message,
    }
}

#[update]
#[ic_cdk::export::candid::candid_method(update)]
fn import_comments(blob: Vec<u8>) -> bool {
    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(blob.as_slice());

        reader
            .deserialize::<CommentExport>()
            .flatten() // filters out error values
            .map(|comment_export| comment_export.into())
            .for_each(|comment_input: CommentInput| {
                let channel = channels
                    .entry(comment_input.channel_id.clone().to_string())
                    .or_insert_with(|| Channel::new(comment_input.channel_id.clone().to_string()));
                if let Some(parent_id) = comment_input.clone().parent_id {
                    let _ = channel.upsert_comment(
                        CommentInput {
                            id: parent_id,
                            ..Default::default()
                        },
                        None,
                    );
                }
                let _ = channel.upsert_comment(comment_input, None);
            });
        true
    })
}

#[init]
pub fn init() -> () {
    println!("TREASURY_CANISTER_ID: {}", TREASURY_CANISTER_ID);
}
