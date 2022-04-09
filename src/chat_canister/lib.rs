use chat_engine::{Channel, CommentInput, CommentOutput as Comment, Page};
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize, Nat};
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CHANNELS: RefCell<HashMap<String, Channel>> = RefCell::new(HashMap::new());
}

#[update]
fn upsert_comment(param: UpsertCommentParam) -> String {
    let user_id = ic_cdk::caller().to_string();
    match authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        &user_id,
        |channel| {
            let comment_input = CommentInput {
                content: param.message.to_string(),
                id: param.comment_id.clone(),
                parent_id: param.parent_id.clone(),
                user_id: user_id.clone(),
                created_at: time(),
            };
            channel.upsert_comment(comment_input)
        },
    ) {
        Ok(result) => match result {
            Ok(output) => output.id,
            Err(message) => message,
        },
        Err(message) => message,
    }
}

#[update]
fn delete_comment(param: DeleteCommentParam) -> String {
    let user_id = ic_cdk::caller().to_string();
    let result = authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        &user_id,
        |channel| {
            channel.delete_comment(param.comment_id.clone());
        },
    );

    "".to_string()
}

#[query]
fn get_thread(param: GetThreadParam) -> IPage {
    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let channel = channels
            .entry(param.channel_id.to_string())
            .or_insert_with(|| Channel::new(param.channel_id.to_string()));
        let page = channel.get_page(&(param.limit as usize), param.cursor.as_ref());
        page.map(|p| p.into()).unwrap_or_default()
    })
}

fn authenticate_user_and_comment_action<A, T>(
    channel_id: &String,
    comment_id: &String,
    user_id: &String,
    action: A,
) -> Result<T, String>
where
    A: Fn(&mut Channel) -> T,
{
    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let channel = channels
            .entry(channel_id.to_string())
            .or_insert_with(|| Channel::new(channel_id.to_string()));

        let message = match channel.get_comment(comment_id) {
            Some(comment) => {
                if &comment.user_id != user_id {
                    Err("UNAUTHORIZED".to_string())
                } else {
                    Ok(action(channel))
                }
            }
            None => Ok(action(channel)),
        };
        message
    })
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct UpsertCommentParam {
    channel_id: String,
    message: String,
    comment_id: String,
    parent_id: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct DeleteCommentParam {
    channel_id: String,
    comment_id: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct GetThreadParam {
    limit: u8,
    channel_id: String,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ICommentOutput {
    id: String,
    content: String,
    created_at: Nat,
    user_id: String,
    replies: IPage,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct IPage {
    comments: Vec<ICommentOutput>,
    remaining_count: Nat,
}

impl From<Comment> for ICommentOutput {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            content: comment.content,
            created_at: Nat::from(comment.created_at),
            user_id: comment.user_id.to_string(),
            replies: comment.replies.into(),
        }
    }
}

impl From<Page> for IPage {
    fn from(page: Page) -> Self {
        IPage {
            comments: page
                .comments
                .into_iter()
                .map(|comment| comment.into())
                .collect(),
            remaining_count: Nat::from(page.remaining_count),
        }
    }
}

//dfx canister call rust_hello get_thread '(record {limit=10;channel_id="channel_1";cursor=null})'
// dfx canister call rust_hello upsert_comment '(record {channel_id="channel_1";message="hello";comment_id="comment_id_1"})'
