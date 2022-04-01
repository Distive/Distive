test_scripts=("cargo test" "npm run --prefix src/js/sdk test" "npm run --prefix src/js/react test")
# Start DFX 
dfx start --background
# Deploy Canisters with clean state
yes | dfx deploy chat_canister -m reinstall

# Run tests print "success" or "failure" depending on the result
for test in "${test_scripts[@]}"
do
    echo "Running test: $test"
    $test
    if [ $? -eq 0 ]; then
        echo "success"
    else
        echo "failure"
    fi
done


# clean up and shutdown
dfx stop