use indexmap::IndexMap;
use std::fmt;
const DELIMITER: &str = ".";
type Thread = IndexMap<String, Comment>;

pub struct Page {
    pub comments: Vec<CommentOutput>,
    pub remaining_count: u32,
}

pub struct Channel {
    // id: String,
    thread: Thread,
}

impl Channel {
    pub fn new(_id: String) -> Self {
        Channel {
            // id,
            thread: IndexMap::new(),
        }
    }

    fn get_thread_as_page(
        thread: &Thread,
        limit: &usize,
        cursor: Option<&String>,
    ) -> Result<Page, String> {
        let limit = *limit;

        match cursor {
            Some(cursor) => match thread.get_index_of(cursor) {
                Some(cursor_index) => {
                    let comments = thread
                        .values()
                        .skip(cursor_index)
                        .take(limit)
                        .map(|comment| comment.clone().into())
                        .collect::<Vec<CommentOutput>>();
                    let thread_length = thread.len();
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
                let comments = thread
                    .values()
                    .take(limit)
                    .map(|comment| comment.clone().into())
                    .collect::<Vec<CommentOutput>>();
                let thread_length = thread.len();
                let remaining_count =
                    (thread_length - thread_length.min(thread_length.min(limit))) as u32;

                Ok(Page {
                    comments,
                    remaining_count,
                })
            }
        }
    }

    pub fn get_page(&mut self, limit: &usize, cursor: Option<&String>) -> Result<Page, String> {
        let (thread, cursor) = match &cursor {
            Some(hierarchal_id) => {
                let mut hierarchal_ids = Channel::split_comment_id(hierarchal_id);
                match hierarchal_ids.len() {
                    0 => (None, None),
                    1 => (Some(&mut self.thread), Some(hierarchal_ids[0].to_string())),
                    _ => {
                        let cursor = hierarchal_ids.pop();
                        (
                            self.get_thread(&hierarchal_ids.join(DELIMITER)),
                            cursor.map(|c| c.to_string()),
                        )
                    }
                }
            }
            None => (Some(&mut self.thread), None),
        };

        match thread {
            Some(thread) => Channel::get_thread_as_page(thread, limit, cursor.as_ref()),
            None => Err(String::from("CURSOR_NOT_FOUND")),
        }
    }

    fn create_hierarchal_id(parent_id: Option<String>, comment_id: &String) -> String {
        parent_id
            .clone()
            .map(|parent_id| [parent_id, comment_id.clone()].join(DELIMITER))
            .unwrap_or(comment_id.clone())
    }

    pub fn upsert_comment(&mut self, comment_input: CommentInput) -> Result<CommentOutput, String> {
        let thread = match &comment_input.parent_id {
            Some(parent_id) => self.get_thread(parent_id),
            None => Some(&mut self.thread),
        };
        match thread {
            Some(thread) => {
                let comment_id = comment_input.id.clone();
                let parent_id = comment_input.parent_id.clone();
                let hierarchal_id = Self::create_hierarchal_id(parent_id, &comment_id);
                let comment = Comment::new(CommentInput {
                    id: hierarchal_id.clone(),
                    ..comment_input
                });
                thread.insert(comment_id, comment.clone());
                Ok(comment.into())
            }
            None => Err(String::from("PARENT_NOT_FOUND")),
        }
    }

    fn split_comment_id(hierarchal_id: &String) -> Vec<&str> {
        hierarchal_id.split(DELIMITER).collect::<Vec<&str>>()
    }

    fn get_thread(&mut self, comment_id: &String) -> Option<&mut Thread> {
        let comment_ids = Channel::split_comment_id(comment_id);

        match comment_ids.len() {
            0 => None,
            _ => {
                let mut thread = &mut self.thread;
                for id in comment_ids {
                    match thread.get_mut(id) {
                        Some(comment) => {
                            thread = &mut comment.replies;
                        }
                        None => return None,
                    }
                }
                Some(thread)
            }
        }
    }


    pub fn get_comment(&mut self, comment_id: &String) -> Option<CommentOutput> {
        let (thread, cursor) =  {
                let mut hierarchal_ids = Channel::split_comment_id(comment_id);
                match hierarchal_ids.len() {
                    0 => (None, None),
                    1 => (Some(&mut self.thread), Some(hierarchal_ids[0].to_string())),
                    _ => {
                        let cursor = hierarchal_ids.pop();
                        (
                            self.get_thread(&hierarchal_ids.join(DELIMITER)),
                            cursor.map(|c| c.to_string()),
                        )
                    }
                }
            };
            
        match (thread, cursor) {
            (Some(thread), Some(cursor)) => {
                let comment = thread.get(&cursor);
                comment.map(|comment| comment.clone().into())
            }
            _ => None,
        }
    }

    // pub fn prune 

    pub fn delete_comment(&mut self, comment_id: String) {
        let (thread, cursor) = {
                let mut hierarchal_ids = Channel::split_comment_id(&comment_id);
                match hierarchal_ids.len() {
                    0 => (None, None),
                    1 => (Some(&mut self.thread), Some(hierarchal_ids[0])),
                    _ => {
                        let cursor = hierarchal_ids.pop();
                        (
                            self.get_thread(&hierarchal_ids.join(DELIMITER)),
                            cursor,
                        )
                    }
                }
        };

        match (thread, cursor) {
            (Some(thread),Some(cursor)) => {
                thread.remove(cursor);
            }
            _ => (),
        };
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
    replies: Thread,
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
            replies: IndexMap::new(),
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
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: comment_id.clone(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        // let got_comment = channel.get_comment(&comment_id).unwrap();
        let thread = channel.get_page(&1, Some(&comment_id)).unwrap();
        assert_eq!(thread.comments[0].content, "hello");
    }

    #[test]
    fn comment_channel_add_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: "comment_id".to_string(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: "comment_id_2".to_string(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: "comment_id_3".to_string(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        assert_eq!(channel.thread.len(), 3);
    }

    #[test]
    fn comment_channel_delete_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        let comment_id = "comment_id".to_string();
        let comment_id_2 = "comment_id_2".to_string();
        let comment_id_3 = "comment_id_3".to_string();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: comment_id.clone(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: comment_id_2.clone(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: comment_id_3.clone(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel.delete_comment(comment_id.clone());
        assert_eq!(channel.thread.len(), 2);

        channel.delete_comment(comment_id_2.clone());
        assert_eq!(channel.thread.len(), 1);

        channel.delete_comment(comment_id_3.clone());
        assert_eq!(channel.thread.len(), 0);
    }

    #[test]
    fn comment_channel_update_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        let comment_id = "comment_id".to_string();
        let comment_id_2 = "comment_id_2".to_string();
        let comment_id_3 = "comment_id_3".to_string();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                created_at: 0,
                id: comment_id.clone(),
                user_id: "user_id".to_string(),
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                created_at: 0,
                id: comment_id_2.clone(),
                user_id: "user_id".to_string(),
                parent_id: None,
            })
            .unwrap();
        channel
            .upsert_comment(CommentInput {
                content: "hello".to_string(),
                id: comment_id_3.clone(),
                user_id: "user_id".to_string(),
                created_at: 0,
                parent_id: None,
            })
            .unwrap();
        channel.update_comment(&comment_id, "hello world");
        assert_eq!(channel.thread.len(), 3);
        assert_eq!(
            channel.get_page(&1,Some(&comment_id)).unwrap().comments[0].content,
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
        channel.upsert_comment(comment_input.clone()).unwrap();
        channel
            .upsert_comment(CommentInput {
                parent_id: None,
                content: "hello world".to_string(),
                ..comment_input
            })
            .unwrap();

        assert_eq!(channel.thread.len(), 1);
        assert_eq!(
            channel.get_page(&1,Some(&comment_id)).unwrap().comments[0].content,
            "hello world"
        );
    }

    #[test]
    fn get_thread() {
        let mut channel = Channel::new("channel_id".to_string());
        let comment_id = "comment_id".to_string();
        let reply_id = "reply_id".to_string();
        let reply_id_2 = "reply_id_2".to_string();

        let comment_input = CommentInput {
            content: "hello".to_string(),
            parent_id: None,
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
        };
        channel.upsert_comment(comment_input.clone()).unwrap();

        let reply = channel
            .upsert_comment(CommentInput {
                parent_id: Some(comment_id.clone()),
                content: "hello world".to_string(),
                id: reply_id.clone(),
                ..comment_input.clone()
            })
            .unwrap();
        //testing out nested replies
        channel
            .upsert_comment(CommentInput {
                parent_id: Some(reply.id.clone()),
                content: "hello world too".to_string(),
                id: reply_id_2.clone(),
                ..comment_input.clone()
            })
            .unwrap();

        //ensure the reply is not added to top level thread
        assert_eq!(channel.thread.len(), 1);

        let thread = channel.get_thread(&comment_id).unwrap();
        assert_eq!(thread.len(), 1);
        assert_eq!(
            thread[0].id,
            Channel::create_hierarchal_id(Some(comment_id), &reply_id)
        );
        assert_eq!(thread[0].content, "hello world");

        //reply of a reply thread (comment->reply->reply)
        let thread = channel.get_thread(&reply.id).unwrap();
        assert_eq!(thread.len(), 1);
        assert_eq!(
            thread[0].id,
            Channel::create_hierarchal_id(Some(reply.id.clone()), &reply_id_2)
        );
        assert_eq!(thread[0].content, "hello world too");
    }

    #[test]
    fn split_comment_id() {
        let hierarchal_id = ["comment_id", "comment_id2"].join(DELIMITER);
        let split_ids = Channel::split_comment_id(&hierarchal_id);

        assert_eq!(split_ids.len(), 2);
        assert_eq!(split_ids, ["comment_id", "comment_id2"]);

        let hierarchal_id = "comment_id".to_string();
        let split_ids = Channel::split_comment_id(&hierarchal_id);

        assert_eq!(split_ids.len(), 1);
        assert_eq!(split_ids, [&hierarchal_id])
    }

    // fn
}
