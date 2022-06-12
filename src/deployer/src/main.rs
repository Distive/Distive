use base64;
use ic_agent::{agent::AgentConfig, ic_types::Principal, identity::BasicIdentity, Agent};
use ic_utils::interfaces::{ManagementCanister, WalletCanister};
use ring::{
    rand,
    signature::{self, Ed25519KeyPair, KeyPair},
};
fn main() {
    println!("Hello, world!");
}

//take public key as base64 encoded string
fn create_identity(serialized_pk: String) -> BasicIdentity {
    let deserialized_pk = base64::decode(&serialized_pk).expect("Could not deserialize pk");

    let key_pair =
        signature::Ed25519KeyPair::from_pkcs8(&deserialized_pk).expect("Failed to create key pair");
    BasicIdentity::from_key_pair(key_pair)
}

async fn create_wallet() -> Option<()> {
    None
}

fn get_wallet() -> Option<()> {
    None
}

async fn get_chat_canister_wasm() -> Option<Vec<u8>> {
    None
}

fn deploy_chat_canister() -> Option<Principal> {
    None
}

async fn get_wallet_canister_wasm() -> Option<Vec<u8>> {
    None
}

// async fn deploy_wallet_canister(agent: &Agent
// ) -> Option<Principal> {
//   let mgr = ManagementCanister::create(
//       Agent::new(config)
//   )
// }

//returns serialiazed keypair (base64)
fn generate_key_pair() -> String {
    let rng = rand::SystemRandom::new();
    let pk_bytes =
        signature::Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate_pkcs8");
    let pk_bytes_vec = pk_bytes.as_ref().to_owned();
    base64::encode(pk_bytes_vec)
}

fn create_agent(identity: BasicIdentity) -> Agent {
    Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create(
                "https://ic0.app",
            )
            .unwrap(),
        )
        .with_identity(identity)
        .build()
        .expect("Failed to create agent")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_identity_runs_without_panic() {
        let public_key_bytes = generate_key_pair();
        let identity = create_identity(public_key_bytes);
    }

    #[test]
    fn test_create_agent_runs_without_panic() {
        let public_key_bytes = generate_key_pair();
        let identity = create_identity(public_key_bytes);
        let agent = create_agent(identity);
    }
}
