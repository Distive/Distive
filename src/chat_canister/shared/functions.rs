use crate::CHANNELS;
use chat_engine::{context::Context, Channel};
use ic_cdk::caller;
pub fn authenticate_user_and_comment_action<A, T>(
    channel_id: &String,
    comment_id: &String,
    context: Option<Context>,
    action: A,
) -> Result<T, String>
where
    A: Fn(&mut Channel) -> T,
{
    CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        let channel = channels
            .entry(channel_id.to_string())
            .or_insert_with(|| Channel::new(channel_id.to_string()));

        let message = match channel.get_comment(comment_id, context.clone()) {
            Some(comment) => {
                if comment.user_id != caller().to_string() {
                    Err("UNAUTHORIZED".to_string())
                } else {
                    Ok(action(channel))
                }
            }
            None => Ok(action(channel)),
        };
        message
    })
}
