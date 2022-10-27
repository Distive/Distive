pub mod comment;
pub mod context;
pub mod metadata;
pub mod page;
mod thread;
use comment::{Comment, CommentExport, CommentInput, CommentOutput};
use context::Context;
use metadata::{Metadata, MetadataInput};
use page::Page;
use thread::Thread;

const DELIMITER: &str = ".";

#[derive(Default)]
pub struct Channel {
    thread: Thread,
}
#[derive(PartialEq, Debug)]
pub struct ChannelExport {
    comment_id: String,
    comment_content: String,
    user_id: String,
    created_at: u64,
    parent_id: Option<String>,
}

impl Channel {
    pub fn new(_id: String) -> Self {
        Channel {
            thread: Thread::new(),
        }
    }

    fn get_thread_as_page(
        thread: &Thread,
        limit: &usize,
        cursor: Option<&String>,
        context: Option<Context>,
    ) -> Result<Page, String> {
        let limit = *limit;

        match cursor {
            Some(cursor) => match thread.get_index_of(cursor) {
                Some(cursor_index) => {
                    let comments = thread
                        .values()
                        .skip(cursor_index)
                        .take(limit)
                        .map(|comment| comment.clone().to_output(context.clone()))
                        .collect::<Vec<CommentOutput>>();
                    let thread_length = thread.len();
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
                    .map(|comment| comment.to_output(context.clone()))
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

    pub fn get_page(
        &mut self,
        limit: &usize,
        cursor: Option<&String>,
        context: Option<Context>,
    ) -> Result<Page, String> {
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
            Some(thread) => Channel::get_thread_as_page(thread, limit, cursor.as_ref(), context),
            None => Err(String::from("CURSOR_NOT_FOUND")),
        }
    }

    fn create_hierarchal_id(parent_id: Option<String>, comment_id: &String) -> String {
        parent_id
            .clone()
            .map(|parent_id| [parent_id, comment_id.clone()].join(DELIMITER))
            .unwrap_or(comment_id.clone())
    }

    fn split_comment_id(hierarchal_id: &String) -> Vec<&str> {
        hierarchal_id.split(DELIMITER).collect::<Vec<&str>>()
    }

    fn parent_id_from_comment_id(hierarchal_id: &String) -> Option<String> {
        let mut hierarchal_ids = Channel::split_comment_id(hierarchal_id);
        hierarchal_ids.pop();

        match hierarchal_ids.len() {
            0 => None,
            _ => Some(hierarchal_ids.join(DELIMITER)),
        }
    }

    // returns Err if parent not found
    pub fn upsert_comment(
        &mut self,
        comment_input: CommentInput,
        context: Option<Context>,
    ) -> Result<CommentOutput, String> {
        let thread = match &comment_input.parent_id {
            Some(parent_id) => self.get_thread(parent_id),
            None => Some(&mut self.thread),
        };
        match thread {
            Some(thread) => {
                let comment_id = Channel::split_comment_id(&comment_input.id)
                    .pop()
                    .unwrap_or(&comment_input.id)
                    .to_string();
                let parent_id = comment_input.parent_id.clone();
                let hierarchal_id = Self::create_hierarchal_id(parent_id, &comment_id);
                match thread.get_mut(&comment_id) {
                    Some(comment) => {
                        comment.content = comment_input.content;
                        Ok(comment.to_output(context))
                    }
                    None => {
                        let comment = Comment::new(CommentInput {
                            id: hierarchal_id.clone(),
                            ..comment_input
                        });
                        thread.insert(comment_id, comment.clone());
                        Ok(comment.to_output(context))
                    }
                }
            }
            None => Err(String::from("PARENT_NOT_FOUND")),
        }
    }

    /// Transverses down the thread hierarchy based on the full comment id
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

    pub fn get_comment(
        &mut self,
        comment_id: &String,
        context: Option<Context>,
    ) -> Option<CommentOutput> {
        let (thread, cursor) = {
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
                comment.map(|comment| comment.to_output(context))
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
                    (self.get_thread(&hierarchal_ids.join(DELIMITER)), cursor)
                }
            }
        };

        match (thread, cursor) {
            (Some(thread), Some(cursor)) => {
                thread.remove(cursor);
            }
            _ => (),
        };
    }

    pub fn toggle_comment_metadata(&mut self, comment_id: &String, metadata: MetadataInput) {
        let MetadataInput { label, user_id } = metadata;
        let mut hierarchal_ids = Channel::split_comment_id(comment_id);

        if let (Some(comment_id), Some(thread)) = match &hierarchal_ids.len() {
            0 => (None, None),
            _ => {
                let comment_id = hierarchal_ids.pop();
                if hierarchal_ids.len() == 0 {
                    (comment_id, Some(&mut self.thread))
                } else {
                    (comment_id, self.get_thread(&hierarchal_ids.join(DELIMITER)))
                }
            }
        } {
            match thread.get_mut(comment_id) {
                Some(Comment {
                    metadata: Some(metadata),
                    ..
                }) => {
                    metadata.toggle_value(&user_id, &label);
                }
                Some(comment) => {
                    let mut metadata = Metadata::new();
                    metadata.toggle_value(&user_id, &label);
                    comment.metadata = Some(metadata);
                }
                _ => (),
            }
        }
    }

    pub fn export(&self) -> Box<dyn Iterator<Item = CommentExport> + '_> {
        let result = self.thread.iter().flat_map(|(_, comment)| comment.export());
        Box::new(result)
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
            ..Default::default()
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
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: comment_id.clone(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        // let got_comment = channel.get_comment(&comment_id).unwrap();
        let thread = channel.get_page(&1, Some(&comment_id), None).unwrap();
        assert_eq!(thread.comments[0].content, "hello");
    }

    #[test]
    fn comment_channel_add_comment() {
        let mut channel = Channel::new("channel_id".to_string());

        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: "comment_id".to_string(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: "comment_id_2".to_string(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: "comment_id_3".to_string(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
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
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: comment_id.clone(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: comment_id_2.clone(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    id: comment_id_3.clone(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
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
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    created_at: 0,
                    id: comment_id.clone(),
                    user_id: "user_id".to_string(),
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();

        channel
            .upsert_comment(
                CommentInput {
                    content: "hello world".to_string(),
                    created_at: 0,
                    id: comment_id.clone(),
                    user_id: "user_id".to_string(),
                    parent_id: None,
                    ..Default::default()
                },
                None,
            )
            .unwrap();
        assert_eq!(channel.thread.len(), 1);
        assert_eq!(
            channel
                .get_page(&1, Some(&comment_id), None)
                .unwrap()
                .comments[0]
                .content,
            "hello world"
        );

        //upserts should not remove replies

        //reply to first comment
        channel
            .upsert_comment(
                CommentInput {
                    content: "reply".to_string(),
                    id: "reply_id".to_string(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: Some(comment_id.clone()),
                    ..Default::default()
                },
                None,
            )
            .unwrap();

        let reply_id =
            &Channel::create_hierarchal_id(Some(comment_id.clone()), &"reply_id".to_string());
        //upsert to reply
        channel
            .upsert_comment(
                CommentInput {
                    content: "updated reply".to_string(),
                    id: reply_id.to_string(),
                    user_id: "user_id".to_string(),
                    created_at: 0,
                    parent_id: Some(comment_id.clone()),
                    ..Default::default()
                },
                None,
            )
            .unwrap();

        let updated_comment = channel.get_comment(reply_id, None).unwrap();
        assert_eq!(updated_comment.content, "updated reply");
        //check that another reply was not created for comment_id
        assert_eq!(channel.thread[0].replies.len(), 1);
        assert_eq!(channel.thread.len(), 1);
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
            ..Default::default()
        };
        channel.upsert_comment(comment_input.clone(), None).unwrap();
        channel
            .upsert_comment(
                CommentInput {
                    parent_id: None,
                    content: "hello world".to_string(),
                    ..comment_input
                },
                None,
            )
            .unwrap();

        assert_eq!(channel.thread.len(), 1);
        assert_eq!(
            channel
                .get_page(&1, Some(&comment_id), None)
                .unwrap()
                .comments[0]
                .content,
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
            ..Default::default()
        };
        channel.upsert_comment(comment_input.clone(), None).unwrap();

        let reply = channel
            .upsert_comment(
                CommentInput {
                    parent_id: Some(comment_id.clone()),
                    content: "hello world".to_string(),
                    id: reply_id.clone(),
                    ..comment_input.clone()
                },
                None,
            )
            .unwrap();
        //testing out nested replies
        channel
            .upsert_comment(
                CommentInput {
                    parent_id: Some(reply.id.clone()),
                    content: "hello world too".to_string(),
                    id: reply_id_2.clone(),
                    ..comment_input.clone()
                },
                None,
            )
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

    #[test]
    fn toggle_comment_metadata() {
        let mut channel = Channel::new("channel_id".to_string());
        let comment_id = "comment_id".to_string();
        let comment_input = CommentInput {
            content: "hello".to_string(),
            parent_id: None,
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            ..Default::default()
        };
        channel.upsert_comment(comment_input.clone(), None).unwrap();
        channel.toggle_comment_metadata(
            &comment_id,
            MetadataInput {
                label: "upvote".to_string(),
                user_id: "user_id".to_string(),
            },
        );
        let comment = channel
            .get_comment(&comment_id, Some(Context::new(vec!["user_id".to_string()])))
            .unwrap();

        assert_eq!(
            comment.metadata,
            vec![("upvote".to_string(), 1, vec![true])]
        );
    }

    #[test]
    fn export_returns_iterator_single_comment() {
        let mut channel = Channel::new("channel_id".to_string());
        channel
            .upsert_comment(
                CommentInput {
                    content: "hello".to_string(),
                    parent_id: None,
                    created_at: 0,
                    id: "comment_id".to_string(),
                    user_id: "user_id".to_string(),
                    ..Default::default()
                },
                None,
            )
            .unwrap();

        let mut exported_data = channel.export();

        assert_eq!(
            exported_data.next(),
            Some(CommentExport(
                "comment_id".to_string(),
                "hello".to_string(),
                "user_id".to_string(),
                0,
                None,
                "".to_string()
            ))
        );
    }

    #[test]
    fn export_returns_iterator_multiple_comment() {
        let mut channel = Channel::new("channel_id".to_string());
        for x in 0..100 {
            channel
                .upsert_comment(
                    CommentInput {
                        content: "hello".to_string(),
                        parent_id: None,
                        created_at: 0,
                        id: format!("comment_id_{}", x),
                        user_id: "user_id".to_string(),
                        ..Default::default()
                    },
                    None,
                )
                .unwrap();
        }

        let mut exported_data = channel.export();

        for x in 0..100 {
            assert_eq!(
                exported_data.next(),
                Some(CommentExport(
                    format!("comment_id_{}", x),
                    "hello".to_string(),
                    "user_id".to_string(),
                    0,
                    None,
                    "".to_string()
                ))
            );
        }
    }

    #[test]
    fn export_returns_iterator_nested_comment() {
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
            ..Default::default()
        };
        channel.upsert_comment(comment_input.clone(), None).unwrap();

        let reply = channel
            .upsert_comment(
                CommentInput {
                    parent_id: Some(comment_id.clone()),
                    content: "hello world".to_string(),
                    id: reply_id.clone(),
                    ..comment_input.clone()
                },
                None,
            )
            .unwrap();
        //testing out nested replies
        let reply2 = channel
            .upsert_comment(
                CommentInput {
                    parent_id: Some(reply.id.clone()),
                    content: "hello world too".to_string(),
                    id: reply_id_2.clone(),
                    ..comment_input.clone()
                },
                None,
            )
            .unwrap();

        let mut exported_data = channel.export();

        assert_eq!(
            exported_data.next(),
            Some(CommentExport(
                "comment_id".to_string(),
                "hello".to_string(),
                "user_id".to_string(),
                0,
                None,
                "".to_string()
            ))
        );

        assert_eq!(
            exported_data.next(),
            Some(CommentExport(
                reply.id.clone(),
                "hello world".to_string(),
                "user_id".to_string(),
                0,
                Some("comment_id".to_string()),
                "".to_string()
            ))
        );

        assert_eq!(
            exported_data.next(),
            Some(CommentExport(
               reply2.id,
                "hello world too".to_string(),
                "user_id".to_string(),
                0,
                Some(reply.id),
                "".to_string()
            ))
        );
    }

    fn create_mock_channel(comment_count: usize) -> Channel {
        let mut channel = Channel::new("channel_id".to_string());
        for x in 0..comment_count {
            channel
                .upsert_comment(
                    CommentInput {
                        content: format!("hello {}", x),
                        parent_id: None,
                        created_at: 0,
                        id: format!("comment_id_{}", x),
                        user_id: "user_id".to_string(),
                        ..Default::default()
                    },
                    None,
                )
                .unwrap();
        }
        channel
    }

    fn add_mock_replies(channel: &mut Channel, comment_count: usize, reply_count: usize) {
        for x in 0..comment_count {
            for y in 0..reply_count {
                channel
                    .upsert_comment(
                        CommentInput {
                            content: format!("hello {}", y),
                            parent_id: Some(format!("comment_id_{}", x)),
                            created_at: 0,
                            id: format!("reply_id_{}_{}", x, y),
                            user_id: "user_id".to_string(),
                            ..Default::default()
                        },
                        None,
                    )
                    .unwrap();
            }
        }
    }

    #[test]
    fn export_can_skip_comments() {
        let channel = create_mock_channel(100);
        let mut exported_data = channel.export();

        for x in 0..20 {
            assert_eq!(
                exported_data.next(),
                // Some(CommentExport {
                //     content: format!("hello {}", x),
                //     parent_id: None,
                //     created_at: 0,
                //     id: format!("comment_id_{}", x),
                //     user_id: "user_id".to_string(),
                //     ..Default::default()
                // })
                Some(CommentExport(
                    format!("comment_id_{}", x),
                    format!("hello {}", x),
                    "user_id".to_string(),
                    0,
                    None,
                    "".to_string()
                ))
            );
        }

        let mut exported_data = exported_data.skip(20);

        for x in 40..60 {
            assert_eq!(
                exported_data.next(),
                // Some(CommentExport {
                //     content: format!("hello {}", x),
                //     parent_id: None,
                //     created_at: 0,
                //     id: format!("comment_id_{}", x),
                //     user_id: "user_id".to_string(),
                //     ..Default::default()
                // })
                Some(CommentExport(
                    format!("comment_id_{}", x),
                    format!("hello {}", x),
                    "user_id".to_string(),
                    0,
                    None,
                    "".to_string()
                ))
            );
        }
    }

    #[test]
    fn export_can_skip_replies() {
        let mut channel = create_mock_channel(2);
        add_mock_replies(&mut channel, 2, 1);
        // comment0->reply0_0->comment1->reply1_0
        let exported_data = channel.export();
        let mut exported_data = exported_data.skip(2);

        assert_eq!(
            exported_data.next(),
            // Some(CommentExport {
            //     content: "hello 1".to_string(),
            //     parent_id: None,
            //     created_at: 0,
            //     id: "comment_id_1".to_string(),
            //     user_id: "user_id".to_string(),
            //     ..Default::default()
            // })
            Some(CommentExport(
                "comment_id_1".to_string(),
                "hello 1".to_string(),
                "user_id".to_string(),
                0,
                None,
                "".to_string()
            ))
        );

        assert_eq!(
            exported_data.next(),
            // Some(CommentExport {
            //     content: "hello 0".to_string(),
            //     parent_id: Some("comment_id_1".to_string()),
            //     created_at: 0,
            //     id: Channel::create_hierarchal_id(
            //         Some("comment_id_1".to_string()),
            //         &"reply_id_1_0".to_string()
            //     ),
            //     user_id: "user_id".to_string(),
            //     ..Default::default()
            // })
            Some(CommentExport(
                Channel::create_hierarchal_id(
                    Some("comment_id_1".to_string()),
                    &"reply_id_1_0".to_string()
                ),
                "hello 0".to_string(),
                "user_id".to_string(),
                0,
                Some("comment_id_1".to_string()),
                "".to_string()
            ))
        )
    }
}
