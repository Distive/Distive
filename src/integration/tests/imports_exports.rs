use candid::{CandidType, Decode, Encode, Nat};
mod common;
use common::create_chat_canister;
use ic_agent::{export::Principal, identity::BasicIdentity, Agent, Identity};
use serde::Deserialize;

#[tokio::test]
async fn test_canister_created() {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Could not generate a key pair.");
    let identity = BasicIdentity::from_key_pair(
        ring::signature::Ed25519KeyPair::from_pkcs8(key_pair.as_ref())
            .expect("Could not read the key pair."),
    );
    let agent = Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create(
                "http://localhost:4943",
            )
            .unwrap(),
        )
        .with_identity(identity)
        .build()
        .unwrap();

    agent.fetch_root_key().await.unwrap();
    let canister_id = create_chat_canister(&agent).await;
    match canister_id.clone() {
        Ok(id) => println!("Canister created with id: {}", id),
        Err(e) => println!("Error creating canister: {}", e),
    }
    assert!(canister_id.is_ok());
}
