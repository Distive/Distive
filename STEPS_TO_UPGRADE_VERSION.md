## Upgrade steps for future automation

- Make changes
- Build and deploy local treasury canister
- Test that treasury canister deploys chat canister
- Test dependent libraries against that chat canister
- Increment version number of dependents based on dependency hierarchy
- Generate changelog
- Upload chat canister wasm and treasury canister wasm to Github
- Deploy treasury canister to the IC


Dependency Hierarchy
Chat Engine -> Chat Canister -> Treasury Canister -> SDK -> ReactJS Library
Updates to any in the chain probably requires updates to the subsequent dependents, so we need to also increment the version numbers of those dependents. 

Files to copy
After building chat canister, copy the following files to the sdk/declaration/ directory:
<!-- Files ending with .did, .ts, and .js -->
`cp .dfx/local/canisters/chat_canister/*.{did,ts,js} ./src/js/sdk/declarations/chat_canister/`