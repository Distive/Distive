//tests
use chat_engine::{Channel, CommentInput};

#[test]
/*
This test ensures the correct behaviour of getting paginated comments (threads)
*/
fn comment_channel_get_comments() {
    let mut channel = Channel::new("channel_id".to_string());

    let empty_thread = channel.get_page(&10,None).unwrap();
    assert_eq!(empty_thread.comments.len(), 0);
    assert_eq!(empty_thread.remaining_count, 0);


    let comment_id = "comment_id".to_string();
    let comment_id_2 = "comment_id_2".to_string();
    let comment_id_3 = "comment_id_3".to_string();

    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id.clone(),
        user_id: "user_id".to_string(),
        parent_id: None,
    }).unwrap();
    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id_2.clone(),
        user_id: "user_id".to_string(),
        parent_id: None,
    }).unwrap();
    channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id_3.clone(),
        user_id: "user_id".to_string(),
        parent_id: None,
    }).unwrap();

    //when cursor is set to the first comment

    //when limit is higher than the number of comments, get all comments after the cursor
    let thread = channel.get_page(&10, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 2);
    //first comment is the comment after cursor comment
    assert_eq!(thread.comments[0].id, comment_id_2);
    //since we got all comments, remaining_count should be zero
    assert_eq!(thread.remaining_count, 0);

    //when limit is lower than the number of comments, get the limit number of comments after the cursor
    let thread = channel.get_page(&2, Some(&comment_id)).unwrap();
    assert_eq!(thread.comments.len(), 2);
    assert_eq!(thread.comments[0].id, comment_id_2);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&10, Some(&comment_id_2)).unwrap();
    assert_eq!(thread.comments.len(), 1);

    assert_eq!(thread.comments[0].id, comment_id_3);
    assert_eq!(thread.remaining_count, 0);

    let thread = channel.get_page(&10, Some(&comment_id_3)).unwrap();
    assert_eq!(thread.comments.len(), 0);
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

   let comment = channel.upsert_comment(CommentInput {
        content: "hello".to_string(),
        created_at: 0,
        id: comment_id.clone(),
        user_id: "user_id".to_string(),
        parent_id: None,
    }).unwrap();

    println!("{:?}", comment.id.clone());
    let reply_id = "reply_id".to_string();

    let reply = channel.upsert_comment(CommentInput {
        content: "hi".to_string(),
        created_at: 0,
        id: reply_id.clone(),
        user_id: "user_id".to_string(),
        parent_id: Some(comment.id.clone()),
    }).unwrap();

    

    let thread = channel.get_page(&10, None).unwrap();
    let first_comment = &thread.comments[0];
    let replies_page = &first_comment.replies;

    assert_eq!(replies_page.comments.len(), 1);
    assert_eq!(replies_page.comments[0].content, "hi");
    assert_eq!(replies_page.remaining_count, 0);

    //add a second reply and test pagination
    let reply_id_2 = "reply_id_2".to_string();
    
    channel.upsert_comment(CommentInput {
        content: "hi too".to_string(),
        created_at: 0,
        id: reply_id_2.clone(),
        user_id: "user_id".to_string(),
        parent_id: Some(comment.id.clone()),
    }).unwrap();

    let thread = channel.get_page(&10, Some(&replies_page.comments[0].id)).unwrap();

    assert_eq!(thread.comments.len(), 1);
    assert_eq!(thread.comments[0].content, "hi too".to_string());
    assert_eq!(thread.remaining_count, 0);
}

#[test]
fn n_nested_comments(){
    
}