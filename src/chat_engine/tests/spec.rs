//tests
use chat_engine::{
    comment::{CommentInput, CommentOutput},
    Channel,
};

#[test]
/*
This test ensures the correct behaviour of getting paginated comments (threads)
*/
fn comment_channel_get_comments() {
    let mut channel = Channel::new("channel_id".to_string());

    let empty_thread = channel.get_page(&10, None).unwrap();
    assert_eq!(empty_thread.comments.len(), 0);
    assert_eq!(empty_thread.remaining_count, 0);

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
            created_at: 0,
            id: comment_id_3.clone(),
            user_id: "user_id".to_string(),
            parent_id: None,
        })
        .unwrap();

    //when cursor is set to the first comment

    //when limit is higher than the number of comments, get all comments including the cursor as the first item
    let thread = channel.get_page(&10, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 3);
    //second comment is the comment after cursor comment
    assert_eq!(thread.comments[1].id, comment_id_2);
    //since we got all comments, remaining_count should be zero
    assert_eq!(thread.remaining_count, 0);

    //when limit is lower than the number of comments, get the limit number of comments after the cursor
    let thread = channel.get_page(&2, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[1].id, comment_id_2);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&10, Some(&comment_id_2)).unwrap();
    assert_eq!(thread.comments.len(), 2);

    assert_eq!(thread.comments[1].id, comment_id_3);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&10, Some(&comment_id_3)).unwrap();
    assert_eq!(thread.comments.len(), 1);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&10, None).unwrap();
    assert_eq!(thread.comments.len(), 3);
    assert_eq!(thread.comments[0].id, comment_id);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&2, None).unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[0].id, comment_id);
    assert_eq!(thread.remaining_count, 1);

    let thread = channel.get_page(&10, Some(&"wrong_id".to_string()));
    match thread {
        Err(message) => assert_eq!(message, "CURSOR_NOT_FOUND"),
        _ => panic!("should not be able to get thread with wrong id"),
    };
}

#[test]
/*
This test ensures the correct behaviour of getting paginated replies (threads) of a comment
*/
fn comment_reply() {
    let mut channel = Channel::new("channel_id".to_string());

    let comment_id = "comment_id".to_string();
    let reply_id = "reply_id".to_string();
    let reply_id_2 = "reply_id_2".to_string();

    //simple top level comment (with no parent)
    let comment = channel
        .upsert_comment(CommentInput {
            content: "comment 1".to_string(),
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            parent_id: None,
        })
        .unwrap();

    println!("{:?}", comment.id.clone());

    //reply to first comment
    let reply = channel
        .upsert_comment(CommentInput {
            content: "reply 1".to_string(),
            created_at: 0,
            id: reply_id.clone(),
            user_id: "user_id".to_string(),
            parent_id: Some(comment.id.clone()),
        })
        .unwrap();

    //paginated thread of top level comments
    let thread = channel.get_page(&10, None).unwrap();
    let first_comment = &thread.comments[0];

    //paginated thread of replies to the first comment
    let replies_page = &first_comment.replies;

    assert_eq!(replies_page.comments.len(), 1);
    assert_eq!(replies_page.comments[0].content, "reply 1");
    assert_eq!(replies_page.remaining_count, 0);

    //add a second reply and test pagination

    channel
        .upsert_comment(CommentInput {
            content: "reply reply 1".to_string(),
            created_at: 0,
            id: reply_id_2.clone(),
            user_id: "user_id".to_string(),
            parent_id: Some(comment.id.clone()),
        })
        .unwrap();

    let thread = channel
        .get_page(&10, Some(&replies_page.comments[0].id))
        .unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[1].content, "reply reply 1".to_string());
    assert_eq!(thread.remaining_count, 0);
}

#[test]

fn n_nested_comments() {
    //channel.get_page, should also return nested comments
    fn create_nested_comments(depth: usize, channel: &mut Channel) {
        let mut parent_id: Option<String> = None;
        for i in 0..depth {
            let comment_id = format!("comment_id_{}", i);
            let comment = channel
                .upsert_comment(CommentInput {
                    content: format!("comment {}", i),
                    created_at: 0,
                    id: comment_id.clone(),
                    user_id: "user_id".to_string(),
                    parent_id: parent_id.clone(),
                })
                .unwrap();
            parent_id = Some(comment.id.clone());
            // println!("{:?}", parent_id);
        }
    }

    //check if every nested comment has one reply
    fn check_nested_comments(depth: usize, channel: &mut Channel) {
        let mut comment_id: Option<String> = None;
        for i in 0..depth - 1 {
            let first_reply = &channel
                .get_page(&10, comment_id.as_ref())
                .unwrap()
                //first comment of thread
                .comments[0]
                .replies
                //first reply of first comment of thread
                .comments[0];

            assert_eq!(first_reply.content, format!("comment {}", i + 1));
            comment_id = Some(first_reply.id.clone());
        }
    }

    let mut channel = Channel::new("channel_id".to_string());
    create_nested_comments(6, &mut channel);
    check_nested_comments(6, &mut channel);

    //a comment should contain the full thread of replies
    fn check_full_thread_count(comment: &CommentOutput) -> usize {
        let mut count = 1;
        if comment.replies.comments.len() > 0 {
            for reply in &comment.replies.comments {
                count += check_full_thread_count(reply);
            }
        };

        count
    }

    let comment = &channel.get_page(&10, None).unwrap().comments[0];
    assert_eq!(check_full_thread_count(comment), 6);

    //when a comment is deleted, its replies should also be removed
    channel.delete_comment("comment_id_0.comment_id_1".to_string());
    assert_eq!(
        check_full_thread_count(&channel.get_page(&10, None).unwrap().comments[0]),
        1
    );
}

#[test]
fn get_comment_test() {
    let mut channel = Channel::new("channel_id".to_string());

    let comment_id = "comment_id".to_string();
    let reply_id = "reply_id".to_string();

    //simple top level comment (with no parent)
    let comment = channel
        .upsert_comment(CommentInput {
            content: "comment 1".to_string(),
            created_at: 0,
            id: comment_id.clone(),
            user_id: "user_id".to_string(),
            parent_id: None,
        })
        .unwrap();

    //reply to first comment
    let reply = channel
        .upsert_comment(CommentInput {
            content: "reply 1".to_string(),
            created_at: 0,
            id: reply_id.clone(),
            user_id: "user_id".to_string(),
            parent_id: Some(comment.id.clone()),
        })
        .unwrap();

    //get the comment
    let comment = channel.get_comment(&comment_id).unwrap();
    assert_eq!(comment.content, "comment 1".to_string());

    //get the reply
    let reply = channel.get_comment(&reply.id).unwrap();
    assert_eq!(reply.content, "reply 1".to_string());

    //get a comment that does not exist
    let comment = channel.get_comment(&"wrong_id".to_string());

    if let Some(_) = comment {
        panic!("should have returned None");
    }
}
