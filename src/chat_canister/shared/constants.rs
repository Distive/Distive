use std::env;
use std::option_env;

pub const TREASURY_CANISTER_ID: &'static str = {
    match option_env!("TREASURY_CANISTER_ID_DEV") {
        Some(id) => {
            if id.is_empty() {
                env!("TREASURY_CANISTER_ID")
            } else {
              id
            }
        }
        _ => env!("TREASURY_CANISTER_ID"),
    }
};

