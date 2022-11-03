use candid::{CandidType, Decode, Encode, Nat};
use ic_agent::{export::Principal, Agent,};
use serde::Deserialize;

#[cfg(test)]
mod tests {
     
    use chat_canister::shared::types::UpsertCommentParam;

    use super::*;

    const CHAT_CANISTER_ID: &str =
        option_env!("CHAT_CANISTER_ID").expect("CHAT_CANISTER_ID must be set to run tests");

    async fn seed_canister(agent: &Agent) {
        let canister_id = Principal::from_text(CHAT_CANISTER_ID).unwrap();
        let mut call = agent.update(&canister_id, "upsert_comment").with_arg(
            Encode!(&UpsertCommentParam {
                channel_id: "test".to_string(),
                comment_id: "test".to_string(),
                parent_id: None,
                message: "test".to_string(),
            })
            .unwrap(),
        );
    }

    async fn clear_canister() {
        // Clear
    }

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
}
