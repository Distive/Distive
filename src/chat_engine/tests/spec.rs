//tests
use chat_engine::{Channel, CommentInput};

#[test]
fn comment_channel_get_comments() {
    let mut channel = Channel::new("channel_id".to_string());

    let comment_id = "comment_id".to_string();
    let comment_id_2 = "comment_id_2".to_string();
    let comment_id_3 = "comment_id_3".to_string();

    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id.clone(),
        user_id: "user_id".to_string(),
    });
    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id_2.clone(),
        user_id: "user_id".to_string(),
    });
    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id_3.clone(),
        user_id: "user_id".to_string(),
    });

    //when cursor is set to the first comment

    //when limit is higher than the number of comments, get all comments after the cursor
    let thread = channel.get_thread(&10, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 2);
    //first comment is the cursor
    assert_eq!(thread.comments[0].id, comment_id_2);
    //since we got all comments, remaining_count should be zero
    assert_eq!(thread.remaining_count, 0);

    //when limit is lower than the number of comments, get the limit number of comments after the cursor
    let thread = channel.get_thread(&2, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[0].id, comment_id_2);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_thread(&10, Some(&comment_id_2)).unwrap();
    assert_eq!(thread.comments.len(), 1);

    assert_eq!(thread.comments[0].id, comment_id_3);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_thread(&10, Some(&comment_id_3)).unwrap();
    assert_eq!(thread.comments.len(), 0);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_thread(&10, None).unwrap();
    assert_eq!(thread.comments.len(), 3);
    assert_eq!(thread.comments[0].id, comment_id);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_thread(&2, None).unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[0].id, comment_id);
    assert_eq!(thread.remaining_count, 1);

    let thread = channel.get_thread(&10, Some(&"wrong_id".to_string()));
    match thread {
        Err(message) => assert_eq!(message, "CURSOR_NOT_FOUND"),
        _ => panic!("should not be able to get thread with wrong id"),
    };
}
