# rm -r .dfx
# start environment for running tests
dfx start --background
dfx_pid=$!
# build canisters
./build.sh
# start treasury canister
echo "yes" | dfx deploy treasury_canister --mode reinstall
export TREASURY_CANISTER_ID_DEV=$(dfx canister id treasury_canister)
echo "Treasury canister id: $TREASURY_CANISTER_ID_DEV"
cd ./src/integration && cargo test -- --nocapture
while :; do sleep 2073600; done
kill $dfx_pid