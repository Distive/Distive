use chat_engine::{comment::CommentOutput as Comment, metadata::MetadataOutput, page::Page};

use ic_cdk::export::candid::{CandidType, Deserialize, Nat};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UpsertCommentParam {
    pub channel_id: String, 
    pub message: String,
    pub comment_id: String,
    pub parent_id: Option<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct DeleteCommentParam {
    pub channel_id: String,
    pub comment_id: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GetThreadParam {
    pub limit: u8,
    pub channel_id: String,
    pub cursor: Option<String>,
    /// The user ids to be checked for metadata.
    pub metadata_user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ExportParam {
    pub cursor: u16
}

#[derive(CandidType, Deserialize)]
pub struct ExportChunk {
   pub data: Vec<u8>,
   pub next_cursor: Option<u16>
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ICommentOutput {
    id: String,
    content: String,
    created_at: Nat,
    user_id: String,
    replies: IPage,
    metadata: MetadataOutput,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct IPage {
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
            metadata: comment.metadata,
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
