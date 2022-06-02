#!/bin/bash
# ENV must be set to  empty string to deploy to ic
export TREASURY_CANISTER_ID_DEV=""
export TREASURY_CANISTER_ID="g3lop-baaaa-aaaag-aaklq-cai"
dfx deploy chat_canister --network ic &&
dfx deploy treasury_canister --network ic