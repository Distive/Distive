#[derive(Default,Clone)]
pub struct Context {
    /// The user ids to be checked for metadata.
    pub user_ids: Vec<String>,
}

impl Context {
    pub fn new(user_ids: Vec<String>) -> Self {
        Context {
            user_ids,
        }
    }
}