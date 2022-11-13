use candid::{CandidType, Decode, Encode, Nat};
mod common;
use common::{create_chat_canister, generate_fake_comment, seed_canister, UpsertCommentParam, FakeCommentIter};
use ic_agent::{export::Principal, identity::BasicIdentity, Agent, Identity};
use serde::Deserialize;

use crate::common::create_agent;

#[tokio::test(flavor = "multi_thread")]
async fn test_canister_created() {
    let agent = create_agent().await;
    let canister_id = create_chat_canister(&agent).await;
    match canister_id.clone() {
        Ok(id) => println!("Canister created with id: {}", id),
        Err(e) => println!("Error creating canister: {}", e),
    }
    assert!(canister_id.is_ok());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_canister_seeding() {
    let agent = create_agent().await;
    let canister_id = create_chat_canister(&agent)
        .await
        .expect("Canister creation failed");
    let comments: Vec<UpsertCommentParam> = FakeCommentIter::new().take(100).collect();
    seed_canister(&agent, &canister_id, comments.clone()).await;
}
