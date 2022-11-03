#!/usr/local/bin/ic-repl
import chat = "rrkah-fqaaa-aaaaa-aaaaq-cai";
let _ = call chat.upsert_comment(
    record {
        channel_id = "channel_1";
        message = "channel_1 message";
        comment_id = "comment_1";
        parent_id = null;
    }
);

function call_toggle_metadata(){
 call chat.toggle_metadata(
    record {
        channel_id = "channel_1";
        comment_id = "comment_1";
        label = "label_1";
    }
);
};

// Anonymous user should be false
assert call_toggle_metadata() == false;

// Authenticated user should be true
identity alice;
assert call_toggle_metadata() == true;

let result = call chat.get_thread(
    record  {
    limit = 1;
    channel_id = "channel_1";
    cursor = null;
    }
);

assert result.comments[0].metadata[0][2] == true;