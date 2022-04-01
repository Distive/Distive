!!!warning
Distive is still early alpha software
!!!
# Installation
Upgrade wallet canister wasm module
dfx wallet --network ic upgrade 

# Add current principal as controller to ic wallet
 dfx canister --network ic --wallet "$(dfx identity --network ic get-wallet)" update-settings --all --add-controller "$(dfx identity get-principal)"

# Deploy Distive canister code
dfx deploy chat_canister --network ic

# Note down canister address
rofub-iaaaa-aaaai-ab7da-cai