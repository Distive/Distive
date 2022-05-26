#[derive(Default,Clone)]
pub struct Context {
    pub current_user_id: String,
}

impl Context {
    pub fn new(current_user_id: String) -> Self {
        Context {
            current_user_id,
        }
    }
}