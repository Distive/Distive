mkdir -p .logs
# exec 1> /dev/null
exec 2> .logs/integration_error.log

START_TIME=$SECONDS
rm -r .dfx
# start environment for running tests
dfx start --background


dfx ledger fabricate-cycles --canister $(dfx identity get-wallet)
dfx canister create treasury_canister
dfx ledger fabricate-cycles --canister treasury_canister


dfx identity new test --disable-encryption
dfx identity use test
dfx_pid=$!

./build.sh
# start treasury canister
echo "yes" | dfx deploy treasury_canister --mode reinstall
export TREASURY_CANISTER_ID_DEV=$(dfx canister id treasury_canister)
echo "Treasury canister id: $TREASURY_CANISTER_ID_DEV"
cd ./src/integration && cargo test -- --nocapture

ELAPSED_TIME=$(($SECONDS - $START_TIME))
echo "Total time: $ELAPSED_TIME seconds"

while :; do sleep 2073600; done
kill $dfx_pid
dfx identity use default

