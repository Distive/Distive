use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt;

const DELIMITER: &str = ".";
const DEFAULT_CHUNK_SIZE: usize = 20;
/// A thread is a linked list of thread chunks, a thread chunk is a list of comments
/// `thread -> thread_chunk_1, thread_chunk_2, ... thread_chunk_n`
/// `thread_chunk -> comment_1, comment_2, ... comment_n`
/// thread chunks are never deleted, only individual comments are deleted from them
type ThreadChunkID = String;

#[derive(PartialEq, Debug, Clone)]
struct ThreadChunk {
    comments: IndexMap<CommentID, Comment>,
    next: Option<ThreadChunkID>,
}
/// A comment is identified by a unique CommentID
///
/// The structure of a CommentID is:
/// ThreadChunkID<DELIMITER>generic_comment_id
/// generic_comment_id is a unique id for a comment that hasn't been assigned to a thread chunk yet.
///
///  The structure of a ThreadChunkID is:
/// channel_id<CHUNK_NO> | channel_id<CHUNK_NO><DELIMETER>CommentID
/// The number after the CommentID signifies the nth chunk of the thread. While each subsequent CommentID after the <DELIMTER> are the replies to the prior CommentID

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct CommentID {
    value: String,
}

impl CommentID {
    pub fn from_generic_id(value: String, thread_chunk_id: String) -> CommentID {
        CommentID {
            value: format!("{}{}{}", thread_chunk_id, DELIMITER, value),
        }
    }

    pub fn thread_chunk_id(&self) -> Option<String> {
        let split = self.value.split(DELIMITER).collect::<Vec<&str>>();
        match split.len() {
            0 | 1 => None,
            _ => Some(split[..split.len() - 1].join(DELIMITER).to_string()),
        }
    }
}

/// The data structure of distive chat engine is a flat map of thread chunks. This map is intended to be distributed in a DHT network
pub struct ThreadMap {
    threads: HashMap<ThreadChunkID, ThreadChunk>,
    chunk_size: usize,
}

impl Default for ThreadMap {
    fn default() -> Self {
        ThreadMap {
            threads: HashMap::new(),
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

/// The id here should not be mistaken as a CommentID. It is a generic id that is not associated to a thread chunk yet.
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

impl ThreadMap {
    pub fn new(chunk_size: usize) -> ThreadMap {
        ThreadMap {
            threads: HashMap::new(),
            chunk_size,
        }
    }

    fn get_thread(&self, chunk_id: &ThreadChunkID) -> Option<&ThreadChunk> {
        self.threads.get(chunk_id)
    }
    fn get_thread_mut(&mut self, chunk_id: &ThreadChunkID) -> Option<&mut ThreadChunk> {
        self.threads.get_mut(chunk_id)
    }

    fn set_thread(&mut self, chunk_id: &ThreadChunkID, chunk: ThreadChunk) {
        self.threads.insert(chunk_id.to_string(), chunk);
    }

    pub fn get_comment(&self, comment_id: &String) -> Option<CommentOutput> {
        let comment_id = CommentID::from_generic_id(comment_id.to_string(), "".to_string());
        let thread_chunk_id = comment_id.thread_chunk_id();
        match thread_chunk_id {
            None => None,
            Some(thread_chunk_id)=>{
                let thread_chunk = self.get_thread(&thread_chunk_id);
                match thread_chunk {
                    None => None,
                    Some(thread_chunk) =>{
                      thread_chunk.comments.get(&comment_id).map(|comment|{
                          CommentOutput {
                              id: comment_id.clone(),
                              content: comment.content.clone(),
                              user_id: comment.user_id.clone(),
                              created_at: comment.created_at,
                              modified_at: comment.modified_at,
                              replies: Page {
                                  comments: self.get_replies(&comment_id),
                              },
                          }
                      })
                    }
                }
            }
        }
    }

    pub fn insert_comment(&mut self, comment_input: CommentInput) -> Result<(), Error> {
        let comment_id =
            CommentID::from_generic_id(comment_input.id.clone(), comment_input.channel_id.clone());

        let thread_chunk_id = comment_id.thread_chunk_id();

        match thread_chunk_id {
            Some(thread_chunk_id) => {
                let thread_chunk = self.get_thread_mut(&thread_chunk_id);
                match thread_chunk {
                    Some(thread_chunk) => {
                        let comment = Comment {
                            id: comment_id.value.clone(),
                            content: comment_input.content.clone(),
                            user_id: comment_input.user_id.clone(),
                            created_at: comment_input.created_at,
                            modified_at: comment_input.modified_at,
                            replies_chunk_id: None,
                        };
                        thread_chunk.comments.insert(comment_id, comment);
                        Ok(())
                    }
                    None => Err(Error::IDNotFound),
                }
            }
            None => Err(Error::IDNotFound),
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunk_map_create() {
        let chunk_map = ThreadMap::default();
        assert_eq!(chunk_map.threads.len(), 0);
    }

    #[test]
    fn chunk_map_set_get() {
        let mut thread_map = ThreadMap::default();
        let chunk_id = &"chunk_id".to_string();
        let chunk = ThreadChunk {
            comments: IndexMap::new(),
            next: None,
        };
        thread_map.set_thread(chunk_id, chunk.clone());
        assert_eq!(thread_map.threads.len(), 1);
        assert_eq!(thread_map.get_thread(chunk_id), Some(&chunk));
    }

    #[test]
    fn can_retrieve_inserted_comment() {
        let mut chunk_map = ThreadMap::default();
        let channel_id = "channel_id".to_string();
        let comment_id = "comment_id".to_string();

        let comment_input = CommentInput {
            id: comment_id.clone(),
            channel_id: channel_id.clone(),
            ..Default::default()
        };

        chunk_map.insert_comment(comment_input).unwrap();
        // assert_eq!(chunk_map.get_comment(&comment_id).unwrap().id, comment_id);
    }

    #[test]
    fn can_retrive() {
        let chunk_map = ThreadMap::default();
        let channel_id = "channel_id".to_string();
        let comment_id = "comment_id".to_string();
    }
}
