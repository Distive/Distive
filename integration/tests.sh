# Start DFX 
dfx start --background
# Deploy Canisters with clean state
dfx deploy rust_hello -m reinstall
# Run Unit Tests (Canisters)
cargo test
# Run Unit Tests (JS SDK)

# Canister <-> SDK Integration Tests

