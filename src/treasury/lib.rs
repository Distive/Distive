use std::cell::RefCell;
use candid::{Principal};
use ic_cdk::{
    api::{
        canister_balance,
    },
    caller,
    export::candid::{CandidType, Deserialize},
    id as current_canister_principal,
};
use ic_cdk_macros::*;
use ic_kit::{
    interfaces::{
        management::{self, CanisterSettings, WithCanisterId},
        Method,
    }
};
const BLACK_HOLE_CANISTER_PRINCIPAL: &str = "e3mmv-5qaaa-aaaah-aadma-cai";

thread_local! {
    pub static CANISTER_COUNT: RefCell<u64> = RefCell::new(0);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Status {
    remaining_cycles: u64,
    canister_count: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateChatCanisterResult {
    success: bool,
    canister_id: String,
    message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Storage {
    canister_count: u64,
}

// impl ArgumentEncoder for Storage {
//     fn encode(self, ser: &mut candid::ser::IDLBuilder) -> candid::Result<()> {

//     }
// }

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn status() -> Status {
    Status {
        remaining_cycles: canister_balance(),
        canister_count: CANISTER_COUNT.with(|c| c.borrow().clone()),
    }
}

// #[pre_upgrade]
// fn pre_upgrade() {
//     let storage = Storage {
//         canister_count: CANISTER_COUNT.with(|c| c.borrow().clone()),
//     };

//     // storage::stable_save(storage).unwrap();
//     stable_store(storage).unwrap();
// }

#[update]
#[ic_cdk::export::candid::candid_method(update)]
async fn wallet_receive() -> () {
    let amount = ic_cdk::api::call::msg_cycles_available();
    if amount > 0 {
        ic_cdk::api::call::msg_cycles_accept(amount);
    }
}

#[update]
#[ic_cdk::export::candid::candid_method(update)]
async fn create_chat_canister() -> CreateChatCanisterResult {
    let caller_principal = caller();

    if caller() == Principal::anonymous() {
        return CreateChatCanisterResult {
            success: false,
            canister_id: "".to_string(),
            message: "You must be logged in to create a canister".to_string(),
        };
    }

    let arg = management::CreateCanisterArgument {
        settings: Some(CanisterSettings {
            compute_allocation: None,
            controllers: Some(vec![current_canister_principal()]),
            freezing_threshold: None,
            memory_allocation: None,
        }),
    };

    let create_canister_result = management::CreateCanister::perform_with_payment(
        Principal::management_canister(),
        (arg,),
        100_000_000_000 + 80_000_590_000 + 100_000_000_000, // creation fee + installation fee + free cycles
    )
    .await;

    let install_code_result = match create_canister_result {
        Ok((WithCanisterId { canister_id },)) => {
            let chat_canister_wasm = include_bytes!("chat_canister.wasm");
            let install_args = management::InstallCodeArgument {
                canister_id,
                mode: management::InstallMode::Install,
                wasm_module: chat_canister_wasm.to_vec(),
                arg: Vec::<u8>::new(),
            };

            let result = management::InstallCode::perform_with_payment(
                Principal::management_canister(),
                (install_args,),
                10_000_000,
            )
            .await;

            result
                .map(|_| canister_id)
                .map_err(|(_, message)| format!("Could not install wasm code: {}", message))
        }
        Err((_, message)) => Err(format!("Could not create canister: {}", message)),
    };

    let update_settings_result = match install_code_result {
        Ok(canister_id) => {
            let update_args = management::UpdateSettingsArgument {
                canister_id: canister_id.clone(),
                settings: CanisterSettings {
                    compute_allocation: None,
                    controllers: Some(vec![
                        Principal::from_text(BLACK_HOLE_CANISTER_PRINCIPAL).unwrap(),
                        caller_principal,
                    ]),
                    freezing_threshold: None,
                    memory_allocation: None,
                },
            };

            let result = management::UpdateSettings::perform(
                Principal::management_canister(),
                (update_args,),
            )
            .await;
            result
                .map(|_| canister_id)
                .map_err(|(_, message)| format!("Could not update canister settings: {}", message))
        }
        Err(message) => Err(message),
    };

    match update_settings_result {
        Ok(canister_id) => {
            CANISTER_COUNT.with(|c| {
                let mut c = c.borrow_mut();
                *c += 1;
            });
            CreateChatCanisterResult {
                success: true,
                canister_id: canister_id.to_string(),
                message: "".to_string(),
            }
        }
        Err(message) => CreateChatCanisterResult {
            success: false,
            canister_id: "".to_string(),
            message,
        },
    }
}
