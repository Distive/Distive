#!/usr/local/bin/ic-repl

import chat = "rrkah-fqaaa-aaaaa-aaaaq-cai";
import ledger = "r7inp-6aaaa-aaaaa-aaabq-cai";
import treasury = "rkp4c-7iaaa-aaaaa-aaaca-cai";

let chat_balance_icp = call ledger.account_balance_dfx(record {account = account(chat)});
let _ = call chat.status();
let chat_balance_cycles = _.remaining_cycles;
chat_balance_cycles;

