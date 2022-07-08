use super::CommentOutput;
pub struct Page {
    pub comments: Vec<CommentOutput>,
    pub remaining_count: u32,
}

// default
impl Default for Page {
    fn default() -> Self {
        Page {
            comments: vec![],
            remaining_count: 0,
        }
    }
}