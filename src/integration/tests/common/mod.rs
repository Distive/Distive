use candid::{CandidType, Decode, Deserialize, Encode};
use fake::{Fake, Faker};
use futures::{stream, Stream, StreamExt};
use garcon::Delay;
use ic_agent::{agent, export::Principal, identity::BasicIdentity, Agent};
pub const TREASURY_CANISTER_ID: &str = env!("TREASURY_CANISTER_ID_DEV");

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
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

    let mut update_builder = agent.update(treasury_principal, "create_chat_canister");

    let result = update_builder
        .with_arg(&Encode!(&()).unwrap())
        .call_and_wait(waiter)
        .await
        .expect("Could not create canister");

    let result: CreateChatCanisterResult =
        Decode!(&result, CreateChatCanisterResult).expect("Could not decode result");
    if result.success {
        Ok(Principal::from_text(&result.canister_id).expect("Invalid principal"))
    } else {
        Err(result.message)
    }
}

pub async fn seed_canister(
    agent: &Agent,
    canister_id: &Principal,
    comments: Vec<UpsertCommentParam>,
) {
    let comment_stream = stream::iter(comments);

    comment_stream
        .for_each_concurrent(100, |param| async move {
            let mut update_call = agent.update(canister_id, "upsert_comment");
            update_call.with_arg(Encode!(&param.clone()).expect("Encode Failure"));
            match update_call
                .call_and_wait(
                    garcon::Delay::builder()
                        .timeout(std::time::Duration::from_secs(20))
                        .build(),
                )
                .await
            {
                Ok(_) => println!("Coment seeded: {:?}", param),
                Err(e) => {
                    // throw
                    panic!("Error seeding comment")
                }
            }
        })
        .await;
}

pub fn generate_fake_comment(channel_id: String, parent_id: Option<String>) -> UpsertCommentParam {
    UpsertCommentParam {
        parent_id,
        channel_id,
        comment_id: fake::uuid::UUIDv5.fake(),
        message: fake::faker::lorem::en::Sentence(10..500).fake(),
    }
}

pub struct FakeCommentIter {
    channel_id: String,
    parent_id: Option<String>,
    _counter: u64,
}

impl FakeCommentIter {
    pub fn new() -> Self {
        Self {
            channel_id: Faker.fake(),
            parent_id: None,
            _counter: 0,
        }
    }
}

impl Iterator for FakeCommentIter {
    type Item = UpsertCommentParam;

    fn next(&mut self) -> Option<Self::Item> {
        let comment = generate_fake_comment(self.channel_id.clone(), self.parent_id.clone());
        self._counter += 1;

        if self._counter % 10 >= 0 && self._counter % 10 < 6 {
            self.parent_id = Some(comment.comment_id.clone());
        } else {
            self.parent_id = None
        }

        Some(comment)
    }
}

async fn clear_canister() {
    // Clear
}

pub async fn create_agent() -> Agent {
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
    agent
        .fetch_root_key()
        .await
        .expect("Could not fetch root key");
    agent
}
