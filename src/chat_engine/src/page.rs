use super::CommentOutput;
pub struct Page {
    pub comments: Vec<CommentOutput>,
    pub remaining_count: u32,
}