# run tests
# if pass, build canister code
# copy .dfx/local/canisters/rust_hello/* to sdk/declarations 
# modify sdk/declarations/canisters/rust_hello/index.js appropriately
# build sdk
npm run --prefix src/js/sdk build
npm run --prefix src/js/react build
