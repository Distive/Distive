use indexmap::IndexMap;
use std::fmt;

type Thread = IndexMap<String, Comment>;

pub struct Page {
    pub comments: Vec<CommentOutput>,
    pub remaining_count: u32,
}

pub struct Channel {
    id: String,
    thread: Thread,
}

impl Channel {
    pub fn new(id: String) -> Self {
        Channel {
            id,
            thread: IndexMap::new(),
        }
    }

    pub fn get_thread(&self, limit: &usize, cursor: Option<&String>) -> Result<Page, String> {
        //convert self.thread to an iterator
        //skip cursor times default to 0
        //get limit number of comments from thread iterator
        // return comments as Vec<Comments> in Page
        let limit = *limit;

        match cursor {
            Some(cursor) => match self.thread.get_index_of(cursor) {
                Some(cursor_index) => {
                    let comments = self
                        .thread
                        .values()
                        .skip(cursor_index + 1)
                        .take(limit)
                        .map(|comment| CommentOutput::from(comment.clone()))
                        .collect::<Vec<CommentOutput>>();
                    let thread_length = self.thread.len();
                    //remaining_count = len - ( len, min(len, min(len, limit) + cursor_index))
                    let remaining_count = (thread_length
                        - thread_length.min(thread_length.min(limit) + (cursor_index + 1)))
                        as u32;

                    Ok(Page {
                        comments,
                        remaining_count,
                    })
                }
                None => Err(String::from("CURSOR_NOT_FOUND")),
            },
            None => {
                let comments = self
                    .thread
                    .values()
                    .take(limit)
                    .map(|comment| CommentOutput::from(comment.clone()))
                    .collect::<Vec<CommentOutput>>();
                let thread_length = self.thread.len();
                //remaining_count = len - ( len, min(len, limit) + cursor_index)
                let remaining_count =
                    (thread_length - thread_length.min(thread_length.min(limit))) as u32;

                Ok(Page {
                    comments,
                    remaining_count,
                })
            }
        }
    }

    pub fn upsert_comment(&mut self, comment_input: CommentInput) {
        self.thread
            .insert(comment_input.id.clone(), Comment::new(comment_input));
    }

    pub fn get_comment(&self, id: &str) -> Option<CommentOutput> {
        self.thread
            .get(id)
            .map(|comment| CommentOutput::from(comment.clone()))
    }

    pub fn delete_comment(&mut self, id: &str) {
        self.thread.remove(id);
    }

    pub fn update_comment(&mut self, id: &str, content: &str) {
        if let Some(comment) = self.thread.get_mut(id) {
            comment.content = content.to_string();
        }
    }
}

#[derive(Clone)]
struct Comment {
    id: String,
    content: String,
    user_id: String,

    created_at: u64,
}

pub struct CommentOutput {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Page,
}

impl From<Comment> for CommentOutput {
    fn from(comment: Comment) -> Self {
        CommentOutput {
            id: comment.id,
            content: comment.content,
            user_id: comment.user_id,
            created_at: comment.created_at,
            replies: Page {
                comments: Vec::new(),
                remaining_count: 0,
            },
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
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.content, self.id)
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channel_create() {
        //a new channel should contain an empty comments map
        let channel = Channel::new("channel_id".to_string());
        assert_eq!(channel.thread.len(), 0);
    }

    #[test]
    fn comment_create() {
        let comment = CommentInput {
            content: "hello".to_string(),
            id: "comment_id".to_string(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        };
        let new_comment = Comment::new(comment);
        assert_eq!(new_comment.content, "hello");
    }

    #[test]
    fn get_comment_by_id() {
        let channel_id = "channel_id".to_string();
        let comment_id = "comment_id".to_string();
        let mut channel = Channel::new(channel_id);
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        let got_comment = channel.get_comment(&comment_id).unwrap();
        assert_eq!(got_comment.content, "hello");
    }

    #[test]
    fn comment_channel_add_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: "comment_id".to_string(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: "comment_id_2".to_string(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: "comment_id_3".to_string(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        assert_eq!(channel.thread.len(), 3);
    }

    #[test]
    fn comment_channel_delete_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        let comment_id = "comment_id".to_string();
        let comment_id_2 = "comment_id_2".to_string();
        let comment_id_3 = "comment_id_3".to_string();
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id_2.clone(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id_3.clone(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.delete_comment(&comment_id);
        assert_eq!(channel.thread.len(), 2);

        channel.delete_comment(&comment_id_2);
        assert_eq!(channel.thread.len(), 1);

        channel.delete_comment(&comment_id_3);
        assert_eq!(channel.thread.len(), 0);
    }

    #[test]
    fn comment_channel_update_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        let comment_id = "comment_id".to_string();
        let comment_id_2 = "comment_id_2".to_string();
        let comment_id_3 = "comment_id_3".to_string();
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            created_at: 0,
            id: comment_id_2.clone(),
            user_id: "user_id".to_string(),
            parent_id: None,
        });
        channel.upsert_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id_3.clone(),
            user_id: "user_id".to_string(),
            created_at: 0,
            parent_id: None,
        });
        channel.update_comment(&comment_id, "hello world");
        assert_eq!(channel.thread.len(), 3);
        assert_eq!(
            channel.get_comment(&comment_id).unwrap().content,
            "hello world"
        );
    }
    #[test]
    fn existing_comment_is_updated() {
        let mut channel = Channel::new("channel_id".to_string());
        let comment_id = "comment_id".to_string();
        let comment_input = CommentInput {
            content: "hello".to_string(),
            parent_id: None,
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
        };
        channel.upsert_comment(comment_input.clone());
        channel.upsert_comment(CommentInput {
            parent_id: None,
            content: "hello world".to_string(),
            ..comment_input
        });

        assert_eq!(channel.thread.len(), 1);
        assert_eq!(
            channel.get_comment(&comment_id).unwrap().content,
            "hello world"
        );
    }
}
