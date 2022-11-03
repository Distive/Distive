#!/usr/local/bin/ic-repl
import chat = "rrkah-fqaaa-aaaaa-aaaaq-cai";
identity alice;
let _ = call chat.upsert_comment(
    record {
        channel_id = "channel_1";
        message = "message 1";
        comment_id = "comment_1";
        parent_id = null;
    }
);

let result = call chat.get_thread(
    record {
        limit = 1;
        channel_id = "channel_1";
        cursor = null;
    }
);

assert result.comments[0].content == "message 1";

identity john;
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

// Another user should not be able to update the comment
assert "message 1"  == result.comments[0].content;