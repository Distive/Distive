#!/usr/local/bin/ic-repl -r https://ic0.app
import treasury = "g3lop-baaaa-aaaag-aaklq-cai";
identity alice;
let result = call treasury.create_chat_canister();
result