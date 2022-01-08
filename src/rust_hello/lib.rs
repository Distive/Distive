mod libs;
use libs::channel::{Channel, CommentInput, Comments};

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

//print hello world
fn test() {}
