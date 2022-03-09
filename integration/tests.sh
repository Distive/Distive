# Start DFX 
dfx start --background
# Deploy Canisters with clean state
yes | dfx deploy rust_hello -m reinstall
# Run Unit Tests (Canisters)
cargo test
# Run Unit Tests (JS SDK)
npm run --prefix src/js/sdk test
# Run React unit tests
npm run --prefix src/js/react test
# Canister <-> SDK Integration Tests

# clean up and shutdown
dfx stop