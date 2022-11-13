use std::io::Write;

use candid::{CandidType, Decode, Encode, Nat};
mod common;
use common::{create_chat_canister, seed_canister, FakeCommentIter, UpsertCommentParam};
use futures::StreamExt;
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

#[tokio::test(flavor = "multi_thread")]
async fn test_canister_csv_exports() {
    let agent = create_agent().await;
    let canister_id = create_chat_canister(&agent)
        .await
        .expect("Canister creation failed");
    let comments: Vec<UpsertCommentParam> = FakeCommentIter::new().take(100).collect();
    println!("Seeding canister with {} comments", comments.len());
    seed_canister(&agent, &canister_id, comments.clone()).await;

    #[derive(Deserialize, CandidType)]
    struct ExportChunk {
        data: Vec<u8>,
        next_cursor: Option<u16>,
    }

    #[derive(Deserialize, CandidType)]
    struct ExportParam {
        cursor: u16,
    }

    let mut query_builder = agent.query(&canister_id, "export_comments");
    let stream = futures::stream::iter(0..);

    let mut cursor: Option<u16> = Some(0);
    let mut data: Vec<u8> = Vec::new();

    while let Some(c) = cursor {
        let param = ExportParam { cursor: c };
        let  result = query_builder
            .with_arg(&Encode!(&param).expect("Encode Failure"))
            .call()
            .await
            .expect("Call Failure");
        let result: ExportChunk = Decode!(&result, ExportChunk).expect("Decode Failure");
        data.extend(result.data);
        cursor = result.next_cursor;
    }

    let mut file = std::fs::File::create("export_test.csv").expect("File creation failed");
    file.write_all(&data).expect("File write failed");
    // count the number of lines in the file
    let count = std::str::from_utf8(&data)
        .expect("Invalid UTF-8")
        .lines()
        .count();
        
    assert_eq!(count, comments.len());
}
