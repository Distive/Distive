use indexmap::IndexMap;
use std::fmt;
mod create_id;
const DELIMITER: &str = "_";

pub struct Channel {
    id: String,
    comments: Comments,
}

impl Channel {
    pub fn new(id: String) -> Self {
        Channel {
            id,
            comments: Comments::new(),
        }
    }

    pub fn get_comments(&self) -> &Comments {
        &self.comments
    }

    fn transverse_threads_for_comment<'a>(&self, comments:&'a Comments, id: &str) -> Option<&'a Comment> {
        let ids = id.split('_').collect::<Vec<&str>>();
        match ids.len() {
            0 => None,
            1 => comments.get_comment(ids[0]),
            _ => {
                let thread_ids = &ids[1..];
                let mut comments = comments;
                for thread_id in thread_ids {
                    if let Some(comment) = comments.get_comment(thread_id) {
                        comments = &comment.replies;
                    };
                }
                comments.get_comment(ids[0])
            }
        }
    }

    pub fn get_commment(&self, id: &str) -> Option<&Comment> {
        let top_level_thread = &self.comments;
        self.transverse_threads_for_comment(top_level_thread, id)
    }

    pub fn add_comment(&mut self, comment: CommentInput) {
        self.comments.add_comment(comment);
    }
}

#[derive(Clone)]
struct Comment {
    pub id: String,
    pub content: String,
    replies: Comments,
    // userId: String,
    // id: String,
    // timestamp: String,
}

#[derive(Clone)]
pub struct Comments {
    value: IndexMap<String, Comment>,
}

impl Comments {
    pub fn new() -> Self {
        Comments {
            value: IndexMap::new(),
        }
    }
    pub fn add_comment(&mut self, comment: CommentInput) -> Comment {
        let new_comment = Comment::new(comment);
        self.value
            .insert(new_comment.id.clone(), new_comment.clone());
        new_comment
    }
    pub fn get_comment(&self, id: &str) -> Option<&Comment> {
        self.value.get(id)
    }

    pub fn get_comments(&self) -> &IndexMap<String, Comment> {
        &self.value
    }
}

#[derive(Clone)]
pub struct CommentInput {
    pub content: String,
    pub id: String,
}

impl Comment {
    pub fn new(comment: CommentInput) -> Self {
        Comment {
            id: comment.id,
            content: comment.content,
            replies: Comments::new(),
        }
    }

    pub fn add_reply(&mut self, reply: CommentInput) {
        self.replies.add_comment(CommentInput {
            id: format!("{}_{}", self.id, reply.id),
            ..reply
        });
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
    use create_id::create_id;

    #[test]
    fn channel_create() {
        //a new channel should contain an empty comments map
        let channel = Channel::new(create_id());
        assert_eq!(channel.comments.value.len(), 0);
    }

    #[test]
    fn comment_create() {
        let comment = CommentInput {
            content: "hello".to_string(),
            id: create_id(),
        };
        let new_comment = Comment::new(comment);
        assert_eq!(new_comment.content, "hello");
    }

    #[test]
    fn comment_add_reply() {
        let comment_id = create_id();
        let reply_id = create_id();
        let comment = CommentInput {
            content: "hello".to_string(),
            id: comment_id.clone(),
        };
        let mut new_comment = Comment::new(comment);
        let reply = CommentInput {
            content: "world".to_string(),
            id: reply_id,
        };
        new_comment.add_reply(reply.clone());
        assert_eq!(new_comment.replies.value.len(), 1);
        let (_, reply_new_comment) = new_comment.replies.value.iter().next().unwrap();
        assert_eq!(reply_new_comment.content, reply.content);
    }

    #[test]
    fn get_comment_by_id() {
        let channel_id = create_id();
        let comment_id = create_id();
        let mut channel = Channel::new(channel_id);
        channel.add_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id.clone(),
        });
        let got_comment = channel.get_commment(&comment_id).unwrap();
        assert_eq!(got_comment.content, "hello");
    }

    #[test]
    fn reply_id_should_have_parent_id() {
        let channel_id = create_id();
        let comment_id = create_id();
        let reply_id = create_id();
        let reply_id_2 = create_id();

        let mut channel = Channel::new(channel_id);

        channel.add_comment(CommentInput {
            content: "hello".to_string(),
            id: comment_id.clone(),
        });

        let mut comment = channel.get_commment(&comment_id.clone()).unwrap();
        println!("{}", comment);

        comment.add_reply(CommentInput {
            content: "world".to_string(),
            id: reply_id.clone(),
        });

        let comments = &channel.get_comments().value;

        for (id, comment) in comments.iter() {
            println!("{}", comment);
            let replies = &comment.replies.value;
            for (id, reply) in replies.iter() {
                println!("{}", reply);
            }
        }

        let reply_with_parent_id = format!("{}_{}", comment_id.clone(), reply_id.clone());
        let reply = channel.get_commment(&reply_with_parent_id);
        assert_eq!(reply.unwrap().content, "world".to_string())
    }

    #[test]
    fn comment_channel_add_comment() {
        let mut channel = Channel::new(create_id());

        channel.comments.add_comment(CommentInput {
            content: "hello".to_string(),
            id: create_id(),
        });
        channel.comments.add_comment(CommentInput {
            content: "hello".to_string(),
            id: create_id(),
        });
        channel.comments.add_comment(CommentInput {
            content: "hello".to_string(),
            id: create_id(),
        });
        assert_eq!(channel.comments.value.len(), 3);
    }

    #[test]
    fn top_level_comment_should_not_be_delimited() {
        let mut channel = Channel::new(create_id());
        let comment = CommentInput {
            content: "hello".to_string(),
            id: create_id(),
        };
        channel.comments.add_comment(comment);
        let (comment_id, comment_new_channel) = channel.comments.value.iter().next().unwrap();
        //comment_id should not contain /
    }
}
