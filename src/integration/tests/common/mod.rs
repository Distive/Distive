use candid::{CandidType, Deserialize, Encode};
use dotenv_codegen::dotenv;
use ic_agent::{export::Principal, Agent};

pub const CHAT_CANISTER_ID: &str = dotenv!("CHAT_CANISTER_ID");
pub const TREASURY_CANISTER_ID: &str = dotenv!("TREASURY_CANISTER_ID_DEV");

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpsertCommentParam {
    pub channel_id: String,
    pub message: String,
    pub comment_id: String,
    pub parent_id: Option<String>,
}

pub fn create_chat_canister() -> Option<Principal> {
    
}

async fn seed_canister(agent: &Agent, canister_id: &Principal) {
    let mut call = agent.update(canister_id, "upsert_comment").with_arg(
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
