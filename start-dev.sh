dfx start --background
dfx deploy rust_hello
# generate mock data
./generate_mock_data.sh
npm run --prefix src/js/examples/zonia-next-example dev