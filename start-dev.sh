dfx start --background
dfx_pid=$!
echo "yes" | dfx deploy chat_canister --mode reinstall
./generate_mock_data.sh
npm run --prefix src/js/examples/distive-next-example dev
sleep infinity
kill $dfx_pid
# generate mock data