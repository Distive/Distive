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
let _ = call chat.get_thread(
    record {
        limit = 10:nat8;
        channel_id = "channel_1";
        cursor = null;
    }
);


assert "channel_1 message" == _.comments[0].content;