use chat_engine::{Channel, CommentInput, CommentOutput as Comment, Page};
use ic_cdk::api::time;
// use ic_cdk::export::candid;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

static mut CHANNELS: Option<HashMap<String, Channel>> = None;

#[init]
fn init_function() {
    unsafe {
        CHANNELS = Some(HashMap::new());
    };
}

#[update]
fn upsert_comment(param: UpsertCommentParam) -> bool {
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
            channel
                .upsert_comment(comment_input)
                .map(|_| true)
                .unwrap_or(false)
        },
    ) {
        Ok(result) => result,
        Err(_) => false,
    }
}

#[update]
fn delete_comment(param: DeleteCommentParam) -> bool {
    let user_id = ic_cdk::caller().to_string();
    authenticate_user_and_comment_action(
        &param.channel_id,
        &param.comment_id,
        &user_id,
        |channel| {
            channel.delete_comment(param.comment_id.clone());
            true
        },
    )
    .unwrap_or(false)
}

#[query]
fn get_thread(param: GetThreadParam) -> IPage {
    if let Some(channels) = unsafe { CHANNELS.as_mut() } {
        let channel = channels
            .entry(param.channel_id.to_string())
            .or_insert_with(|| Channel::new(param.channel_id.to_string()));
        let page = channel.get_page(&param.limit, param.cursor.as_ref());
        page.map(|p| p.into()).unwrap_or_default()
    } else {
        IPage {
            comments: vec![],
            remaining_count: 0,
        }
    }
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
    if let Some(channels) = unsafe { CHANNELS.as_mut() } {
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
            None => Err("NOT FOUND".to_string()),
        };
        message
    } else {
        Err("ERR".to_string())
    }
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
    limit: usize,
    channel_id: String,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ICommentOutput {
    id: String,
    content: String,
    created_at: u64,
    user_id: String,
    replies: IPage,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct IPage {
    comments: Vec<ICommentOutput>,
    remaining_count: u32,
}

impl From<Comment> for ICommentOutput {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            content: comment.content,
            created_at: comment.created_at,
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
            remaining_count: page.remaining_count,
        }
    }
}
