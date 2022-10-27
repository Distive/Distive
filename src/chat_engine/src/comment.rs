use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::metadata::{Metadata, MetadataOutput};
use crate::Channel;
use crate::Page;
use crate::Thread;
use core::fmt;
use std::iter::once;

#[derive(Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Thread,
    pub metadata: Option<Metadata>,
    pub channel_id: String,
}

#[derive(Debug)]
pub struct CommentOutput {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Page,
    pub metadata: MetadataOutput,
    pub channel_id: String,
}

// pub struct CommentExport {
//   value: CommentExportInner
// }
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
/// id, content, user_id, created_at, parent_id, channel_id
pub struct CommentExport(pub String, pub String, pub String, pub u64, pub Option<String>, pub String);
// pub struct CommentExport {
//     pub id: String,
//     pub content: String,
//     pub user_id: String,
//     pub created_at: u64,
//     pub parent_id: Option<String>,
//     pub channel_id: String,
// }


#[derive(Clone, Default)]
pub struct CommentInput {
    pub content: String,
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub parent_id: Option<String>,
    pub channel_id: String,
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
            channel_id: comment_input.channel_id,
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
            channel_id: self.channel_id.clone(),
        }
    }

    pub fn export(&self) -> Box<dyn Iterator<Item = CommentExport> + '_> {
        let result = once(self)
            .map(|c| c.into())
            .chain(self.replies.iter().flat_map(|c| c.1.export()));
        Box::new(result)
    }
}



impl From<&Comment> for CommentExport {
    fn from(comment: &Comment) -> Self {
        // CommentExport {
        //     id: comment.id.clone(),
        //     content: comment.content.clone(),
        //     user_id: comment.user_id.clone(),
        //     created_at: comment.created_at,
        //     parent_id: Channel::parent_id_from_comment_id(&comment.id),
        //     channel_id: comment.channel_id.clone(),
        // }
        CommentExport(
            comment.id.clone(),
            comment.content.clone(),
            comment.user_id.clone(),
            comment.created_at,
            Channel::parent_id_from_comment_id(&comment.id),
            comment.channel_id.clone(),
        )
    }
}

impl From<&CommentExport> for CommentInput {
    fn from(comment: &CommentExport) -> Self {
        // CommentInput {
        //     id: comment.id.clone(),
        //     content: comment.content.clone(),
        //     user_id: comment.user_id.clone(),
        //     created_at: comment.created_at,
        //     parent_id: comment.parent_id.clone(),
        //     channel_id: comment.channel_id.clone(),
        // }
        CommentInput {
            id: comment.0.clone(),
            content: comment.1.clone(),
            user_id: comment.2.clone(),
            created_at: comment.3,
            parent_id: comment.4.clone(),
            channel_id: comment.5.clone(),
        }
    }
}

impl From<CommentExport> for CommentInput {
    fn from(comment: CommentExport) -> Self {
        // CommentInput {
        //     id: comment.id,
        //     content: comment.content,
        //     user_id: comment.user_id,
        //     created_at: comment.created_at,
        //     parent_id: comment.parent_id,
        //     channel_id: comment.channel_id,
        // }
        CommentInput {
            id: comment.0,
            content: comment.1,
            user_id: comment.2,
            created_at: comment.3,
            parent_id: comment.4,
            channel_id: comment.5,
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.content, self.id)
    }
}
