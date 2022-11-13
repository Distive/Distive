export TREASURY_CANISTER_ID_DEV=""
export TREASURY_CANISTER_ID="g3lop-baaaa-aaaag-aaklq-cai"
rm ./target/wasm32-unknown-unknown/release/chat_canister.wasm
# cargo build --package chat_canister --release --target=wasm32-unknown-unknown
dfx build chat_canister
cp ./target/wasm32-unknown-unknown/release/chat_canister.wasm ./src/treasury_canister/
