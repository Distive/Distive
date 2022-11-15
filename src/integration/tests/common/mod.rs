use candid::{CandidType, Decode, Deserialize, Encode};
use fake::{Fake, Faker};
use futures::{stream, Stream, StreamExt};
use garcon::Delay;
use ic_agent::{agent, export::Principal, identity::BasicIdentity, Agent};
use rand::distributions::Uniform;
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
                Ok(result) => {
                    let message: String = Decode!(&result, String).expect("Decode Error");
                    // println!("Result: {:?}", message);
                }
                Err(e) => {
                    // throw
                    println!("Result: {:?}", e);
                    panic!("Error seeding comment")
                }
            }
        })
        .await;
}

fn generate_fake_comment(channel_id: String, parent_id: Option<String>) -> UpsertCommentParam {
    UpsertCommentParam {
        parent_id: None,
        channel_id,
        comment_id: fake::uuid::UUIDv5.fake(),
        message: lipsum::lipsum(rand::Rng::gen_range(&mut rand::thread_rng(), 10..100) as usize),
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

pub async fn export_canister_data(agent: &Agent, canister_id: &Principal) -> Vec<u8> {
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

    let mut cursor: Option<u16> = Some(0);
    let mut data: Vec<u8> = Vec::new();

    while let Some(c) = cursor {
        let param = ExportParam { cursor: c };
        let result = query_builder
            .with_arg(&Encode!(&param).expect("Encode Failure"))
            .call()
            .await
            .expect("Call Failure");
        let result: ExportChunk = Decode!(&result, ExportChunk).expect("Decode Failure");
        data.extend(result.data);
        cursor = result.next_cursor;
    }

    data
}
