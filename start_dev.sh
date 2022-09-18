dfx start --background
dfx_pid=$!
# ./start_dev_ledger.sh
export TREASURY_CANISTER_ID_DEV="r7inp-6aaaa-aaaaa-aaabq-cai"
echo "yes" | dfx deploy chat_canister --mode reinstall
echo "yes" | dfx deploy treasury_canister --mode reinstall
# ./generate_mock_data.sh
# npm run --prefix src/js/examples/distive-next-example dev
while :; do sleep 2073600; done
kill $dfx_pid