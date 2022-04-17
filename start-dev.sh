dfx start --background
dfx deploy chat_canister
# generate mock data
./generate_mock_data.sh
npm run --prefix src/js/examples/zonia-next-example dev