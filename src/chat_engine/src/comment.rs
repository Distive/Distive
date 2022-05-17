use crate::metadata::{Metadata, MetadataOutput};
use crate::Channel;
use crate::Page;
use crate::Thread;
use core::fmt;

#[derive(Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Thread,
    pub metadata: Option<Metadata>,
}

pub struct CommentOutput {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Page,
    pub metadata: MetadataOutput,
}

impl From<Comment> for CommentOutput {
    fn from(comment: Comment) -> Self {
        let user_id = comment.user_id.clone();
        CommentOutput {
            metadata: comment.metadata.map_or(vec![], |m| m.to_output(&user_id)),
            id: comment.id,
            content: comment.content,
            user_id,
            created_at: comment.created_at,
            replies: Channel::get_thread_as_page(&comment.replies, &10, None).unwrap_or(Page {
                comments: vec![],
                remaining_count: 0,
            }),
        }
    }
}

#[derive(Clone)]
pub struct CommentInput {
    pub content: String,
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub parent_id: Option<String>,
}

impl Comment {
    pub fn new(comment_input: CommentInput) -> Self {
        Comment {
            id: comment_input.id,
            content: comment_input.content,
            user_id: comment_input.user_id,
            created_at: comment_input.created_at,
            replies: Thread::new(),
            metadata: None,
        }
    }

    // pub fn upsertMetadata(&self, value: Metadata)
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.content, self.id)
    }
}
