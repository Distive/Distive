#!/usr/local/bin/ic-repl
import chat = "rrkah-fqaaa-aaaaa-aaaaq-cai";

identity alice;

let result = call chat.get_thread(
    record  {
    limit = 10;
    channel_id = "main";
    cursor = null;
    }
);

let id = result.comments[0].id;

call chat.toggle_metadata(
    record {
        channel_id = "main";
        comment_id = id;
        label = "up";
    }
);

assert _ == true;