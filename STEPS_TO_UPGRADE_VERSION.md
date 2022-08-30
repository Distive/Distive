## Upgrade steps for future automation
<!-- TODO: install typicode/husky -->
- Make changes
- Build and deploy local treasury canister
- Test that treasury canister deploys chat canister
- Test dependent libraries against that chat canister
- Increment version number of dependents based on dependency hierarchy
- Generate changelog
- Upload chat canister wasm and treasury canister wasm to Github
- Deploy treasury canister to the IC


Dependency Hierarchy
Chat Engine -> Chat Canister -> Treasury Canister -> SDK -> ReactJS Library
Major or Minor (not patch) Updates to any in the chain probably requires updates to the subsequent dependents, so we need to also increment the version numbers of those dependents. 

Files to copy
After building chat canister, copy the following files to the sdk/declaration/ directory:
<!-- Files ending with  .ts, and .js -->
```cp .dfx/local/canisters/chat_canister/chat_canister.did.d.ts ./src/js/packages/sdk/declarations/chat_canister/ &&
cp .dfx/local/canisters/chat_canister/chat_canister.did.js ./src/js/packages/sdk/declarations/chat_canister/ &&
cp .dfx/local/canisters/chat_canister/index.js ./src/js/packages/sdk/declarations/chat_canister/ 
```

Update ./src/js/sdk/declarations/chat_canister/index.js to include the following:

```
export const init_actor = (canisterId, host = "https://boundary.ic0.app/", identity) => {
  const chat_actor = createActor(
    canisterId,
    {
      agentOptions: {
        host,
        identity,
      }
    }
  )

  return chat_actor
}

```

