use chat_engine::{Channel, CommentInput, CommentOutput as Comment};
use ic_cdk::api::time;
use ic_cdk::export::candid;
use ic_cdk_macros::{init, query, update};
static mut Channel: Option<Channel> = None;

#[init]
fn init_function() {
    unsafe {
        Channel = Some(Channel::new("id".to_string()));
    };
}

#[update]
fn comment(channel_id: String, message: String, user_id: String, comment_id: String) -> String {
    if let Some(channel) = unsafe { Channel.as_mut() } {
        let comment_input = CommentInput {
            content: message.to_string(),
            id: comment_id.clone(),
            parent_id: None,
            user_id: user_id.to_string(),
            created_at: time(),
        };
        channel.upsert_comment(comment_input.clone());
        comment_input.id.clone()
    } else {
        "".to_string()
    }
}

// #[query]
// fn get_comment(channel_id: String, comment_id: String) -> String {
//     if let Some(channel) = unsafe { Channel.as_ref() } {
//         if let Ok(thread) = channel.get_page(&1,Some(&comment_id)) {
//           match   thread.comments.get(0) {
//               Some(comment) => comment.content.clone(),
//               None => "NOT FOUND".to_string()
//           }
           
//         } else {
//             "NOT FOUND".to_string()
//         }
//     } else {
//         "CHANNEL NOT INITILIALIZED".to_string()
//     }
// }

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
