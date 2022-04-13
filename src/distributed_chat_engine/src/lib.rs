use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt;

const DELIMITER: &str = ".";
const DEFAULT_CHUNK_SIZE: usize = 20;
/// A thread is a linked list of thread chunks, a thread chunk is a list of comments
/// `thread -> thread_chunk_1, thread_chunk_2, ... thread_chunk_n`
/// `thread_chunk -> comment_1, comment_2, ... comment_n`
/// thread chunks are never deleted, only individual comments are deleted from them

#[derive(PartialEq, Debug, Clone)]
struct ThreadChunk {
    comments: IndexMap<CommentID, Comment>,
    next: Option<ThreadChunkID>,
}
/// A comment is identified by a unique CommentID
/// The structure of a CommentID is:
/// channel_id<DELIMITER>thread_chunk_id_1<DELIMITER>...thread_chunk_id_n<DELIMITER>comment_id
/// The structure of a ThreadChunkID is:
/// channel_id<DELIMITER>thread_chunk_id_1...thread_chunk_id_n (the comment_id is removed)
type CommentID = String;

type ThreadChunkID = String;
/// The data structure of distive chat engine is a flat map of thread chunks. This map is intended to be distributed in a DHT network
pub struct ThreadChunkMap {
    chunks: HashMap<ThreadChunkID, ThreadChunk>,
    chunk_size: usize,
}

impl Default for ThreadChunkMap {
    fn default() -> Self {
        ThreadChunkMap {
            chunks: HashMap::new(),
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Comment {
    id: String,
    content: String,
    user_id: String,
    created_at: u64,
    modified_at: u64,
    replies_chunk_id: Option<ThreadChunkID>,
}

#[derive(Clone, Default, Debug)]
pub struct CommentInput {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub modified_at: u64,
    pub channel_id: String,
    pub parent_id: Option<CommentID>,
}

#[derive(Debug)]
pub struct CommentOutput {
    pub id: CommentID,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Page,
    pub modified_at: u64,
}

#[derive(Debug)]
pub struct Page {
    // pub thread_chunk_id: ThreadChunkID,
    pub comments: Vec<CommentOutput>,
}
#[derive(Debug)]
pub enum Error {
    IDNotFound,
}

impl ThreadChunkMap {
    pub fn new(chunk_size: usize) -> ThreadChunkMap {
        ThreadChunkMap {
            chunks: HashMap::new(),
            chunk_size,
        }
    }

    fn get_chunk(&self, chunk_id: &ThreadChunkID) -> Option<&ThreadChunk> {
        self.chunks.get(chunk_id)
    }
    fn get_chunk_mut(&mut self, chunk_id: &ThreadChunkID) -> Option<&mut ThreadChunk> {
        self.chunks.get_mut(chunk_id)
    }

    fn set_chunk(&mut self, chunk_id: &ThreadChunkID, chunk: ThreadChunk) {
        self.chunks.insert(chunk_id.to_string(), chunk);
    }

    pub fn get_comment(&self, comment_id: &CommentID) -> Option<&CommentOutput> {
        None
    }

    pub fn insert_comment(&mut self, comment_input: CommentInput) -> Result<CommentOutput, Error> {
        Ok(CommentOutput {
            id: "".to_string(),
            content: "".to_string(),
            user_id: "".to_string(),
            created_at: 0,
            modified_at: 0,
            replies: Page { comments: vec![] },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunk_map_create() {
        let chunk_map = ThreadChunkMap::default();
        assert_eq!(chunk_map.chunks.len(), 0);
    }

    #[test]
    fn chunk_map_set_get() {
        let mut chunk_map = ThreadChunkMap::default();
        let chunk_id = &"chunk_id".to_string();
        let chunk = ThreadChunk {
            comments: IndexMap::new(),
            next: None,
        };
        chunk_map.set_chunk(chunk_id, chunk.clone());
        assert_eq!(chunk_map.chunks.len(), 1);
        assert_eq!(chunk_map.get_chunk(chunk_id), Some(&chunk));
    }

    #[test]
    fn can_retrieve_inserted_comment() {
        let mut chunk_map = ThreadChunkMap::default();
        let channel_id = "channel_id".to_string();
        let comment_id = "comment_id".to_string();

        let comment_input = CommentInput {
            id: comment_id.clone(),
            channel_id: channel_id.clone(),
            ..Default::default()
        };

        chunk_map.insert_comment(comment_input).unwrap();
        assert_eq!(chunk_map.get_comment(&comment_id).unwrap().id, comment_id);
    }

    #[test]
    fn get_comment() {
        let chunk_map = ThreadChunkMap::default();
        let channel_id = "channel_id".to_string();
        let comment_id = "comment_id".to_string();
    }
}
