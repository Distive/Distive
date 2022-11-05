use candid::{CandidType, Decode, Encode, Nat};
mod common;
use common::CHAT_CANISTER_ID;
use ic_agent::{export::Principal, Agent};
use serde::Deserialize;


#[test]
fn test_import_export() {
    let agent = Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create(
                "http://localhost:8000",
            )
            .unwrap(),
        )
        .build()
        .unwrap();

    let query_builder = agent.query(
        &Principal::from_text(CHAT_CANISTER_ID).unwrap(),
        "export_comments",
    );
}
