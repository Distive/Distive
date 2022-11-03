#!/usr/local/bin/ic-repl
import chat = "rrkah-fqaaa-aaaaa-aaaaq-cai";

// Should not be able to update unauthenticated comments
let _ = call chat.upsert_comment(
    record {
        channel_id = "channel_1";
        message = "message 1";
        comment_id = "comment_1";
        parent_id = null;
    }
);

let _ = call chat.upsert_comment(
    record {
        channel_id = "channel_1";
        message = "message 2";
        comment_id = "comment_1";
        parent_id = null;
    }
);

let result = call chat.get_thread(
    record  {
    limit = 1;
    channel_id = "channel_1";
    cursor = null;
    }
);

assert "message 1"  == result.comments[0].content;