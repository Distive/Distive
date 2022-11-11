use candid::{CandidType, Decode, Deserialize, Encode};
use fake::{Dummy, Fake, Faker};
use garcon::Delay;
use ic_agent::{agent, export::Principal, Agent};
pub const TREASURY_CANISTER_ID: &str = env!("TREASURY_CANISTER_ID_DEV");

#[derive(Clone, Debug, Default, CandidType, Deserialize, Dummy)]
pub struct UpsertCommentParam {
    // #[dummy(faker = "uuid::Uuid::new_v4()")]
    pub channel_id: String,

    pub message: String,

    pub comment_id: String,

    pub parent_id: Option<String>,
}

pub async fn create_chat_canister(agent: &Agent) -> Result<Principal, String> {
    // Words(2..5).fake();
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(20))
        .build();

    #[derive(Deserialize, CandidType)]
    struct CreateChatCanisterResult {
        success: bool,
        canister_id: String,
        message: String,
    }

    let treasury_principal =
        &Principal::from_text(TREASURY_CANISTER_ID).expect("Invalid principal");

    let mut update_builder = agent
        .update(treasury_principal, "create_chat_canister");


    


    let result = update_builder
        .with_arg(&Encode!(&()).unwrap())
        .call_and_wait(waiter)
        .await
        .expect("Could not create canister");

    let result: CreateChatCanisterResult = Decode!(&result, CreateChatCanisterResult).expect("Could not decode result");
    if result.success {
        Ok(Principal::from_text(&result.canister_id).expect("Invalid principal"))
    } else {
        Err(result.message)
    }

}

async fn seed_canister(agent: &Agent, canister_id: &Principal) {
    // let mut call = agent.update(canister_id, "upsert_comment").with_arg(
    //     Encode!(&UpsertCommentParam {
    //         channel_id: "test".to_string(),
    //         comment_id: "test".to_string(),
    //         parent_id: None,
    //         message: "test".to_string(),
    //     })
    //     .unwrap(),
    // );

    for _ in 0..100 {
        let param = generate_fake_comment();
        let mut update_call = agent.update(canister_id, "upsert_comment");
        update_call.with_arg(Encode!(&param).unwrap());
        let result = update_call
            .call_and_wait(
                garcon::Delay::builder()
                    .timeout(std::time::Duration::from_secs(20))
                    .build(),
            )
            .await;
        // results.push(result);
    }
}

fn generate_fake_comment() -> UpsertCommentParam {
    Faker.fake()
}

async fn clear_canister() {
    // Clear
}
