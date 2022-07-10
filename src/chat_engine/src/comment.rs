use crate::context::Context;
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

    pub fn to_output(&self, context: Option<Context>) -> CommentOutput {
        let user_ids = context.unwrap_or_default().user_ids;

        CommentOutput {
            metadata: self
                .metadata
                .as_ref()
                .map_or(vec![], |m| m.to_output(&user_ids)),
            id: self.id.clone(),
            content: self.content.clone(),
            user_id: self.user_id.clone(),
            created_at: self.created_at,
            replies: Channel::get_thread_as_page(&self.replies, &10, None, None)
                .unwrap_or_default(),
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.content, self.id)
    }
}
