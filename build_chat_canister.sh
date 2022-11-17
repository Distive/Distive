export TREASURY_CANISTER_ID_DEV=""
export TREASURY_CANISTER_ID="g3lop-baaaa-aaaag-aaklq-cai"
rm ./src/treasury_canister/chat_canister.wasm
rm ./target/wasm32-unknown-unknown/release/chat_canister.wasm
# cargo build --package chat_canister --release --target=wasm32-unknown-unknown
dfx canister create chat_canister
dfx build chat_canister
cp ./target/wasm32-unknown-unknown/release/chat_canister.wasm ./src/treasury_canister/
