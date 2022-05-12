use crate::Channel;
use crate::Page;
use crate::Thread;

use hashbrown::HashMap;
use hashbrown::HashSet;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Thread,
    pub metadata: Option<Metadata>,
}

#[derive(Default,Clone)]
pub struct Metadata {
    pub value: HashMap<String, HashSet<String>>,
}

impl Metadata {
    
    fn add(&mut self, user_id: String, label: String) {
        self.value
            .entry(label)
            .or_insert(HashSet::new())
            .insert(user_id);
    }

    fn remove(&mut self, user_id: &String, label: &String) {
        self.value.entry(label.to_string()).and_modify(|v| {
            v.remove(user_id);
        });
    }

    fn exists(&self, user_id: &String, label: &String) -> bool {
        self.value
            .get(label)
            .map(|v| v.contains(user_id))
            .unwrap_or(false)
    }

    pub fn toggle_value(&mut self, user_id: &String, label: &String) {
        if self.exists(user_id, label) {
            self.remove(user_id, label);
        } else {
            self.add(user_id.to_string(), label.to_string());
        }
    }

    pub fn user_count(&mut self) -> Vec<(String, usize)> {
        self.value
            .iter()
            .map(|(label, users)| (label.to_string(), users.len()))
            .collect()
    }
}


impl Deref for Metadata {
    type Target = HashMap<String, HashSet<String>>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}


pub struct CommentOutput {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub created_at: u64,
    pub replies: Page,
}

impl From<Comment> for CommentOutput {
    fn from(comment: Comment) -> Self {
        CommentOutput {
            id: comment.id,
            content: comment.content,
            user_id: comment.user_id,
            created_at: comment.created_at,
            replies: Channel::get_thread_as_page(&comment.replies, &10, None).unwrap_or(Page {
                comments: vec![],
                remaining_count: 0,
            }),
        }
    }
}

#[derive(Clone)]
pub struct CommentInput {
    pub content: String,
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub parent_id: Option<String>,
}

impl Comment {
    pub fn new(comment_input: CommentInput) -> Self {
        Comment {
            id: comment_input.id,
            content: comment_input.content,
            user_id: comment_input.user_id,
            created_at: comment_input.created_at,
            replies: Thread::new(),
            metadata: None,
        }
    }

    // pub fn upsertMetadata(&self, value: Metadata)
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.content, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_metadata_takes_a_label_and_user_id() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 1);

        //user 2
        metadata.add("user_id2".to_string(), "label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 2);
    }

    #[test]
    fn remove_metadata_takes_a_label_and_user_id() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        metadata.add("user_id2".to_string(), "label".to_string());

        metadata.remove(&"user_id".to_string(), &"label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 1);

        metadata.remove(&"user_id2".to_string(), &"label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 0);
    }

    #[test]
    fn toggle_metadata_adds_and_removes() {
        let mut metadata = Metadata::default();
        metadata.toggle_value(&"user_id".to_string(), &"label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 1);
        metadata.toggle_value(&"user_id".to_string(), &"label".to_string());
        assert_eq!(metadata.get("label").unwrap().len(), 0);
    }

    #[test]
    fn metadata_can_have_multiple_labels_per_user_id() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        metadata.add("user_id".to_string(), "label2".to_string());
      
        assert_eq!(metadata.get("label").unwrap().len(), 1);
        assert_eq!(metadata.get("label2").unwrap().len(), 1);
    }

    #[test]
    fn metadata_can_have_multiple_user_ids_per_label() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        metadata.add("user_id2".to_string(), "label".to_string());
       
        assert_eq!(metadata.get("label").unwrap().len(), 2);
    }

    #[test]
    fn metadata_can_have_multiple_labels_per_user_id_and_multiple_user_ids_per_label() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        metadata.add("user_id".to_string(), "label2".to_string());
        metadata.add("user_id2".to_string(), "label".to_string());
        metadata.add("user_id2".to_string(), "label2".to_string());
    
        assert_eq!(metadata.get("label").unwrap().len(), 2);
        assert_eq!(metadata.get("label2").unwrap().len(), 2);
    }

    #[test]
    fn can_count_number_of_users_per_label() {
        let mut metadata = Metadata::default();
        metadata.add("user_id".to_string(), "label".to_string());
        metadata.add("user_id".to_string(), "label2".to_string());

        let counts = metadata.user_count();

        assert_eq!(counts.len(), 2);
        assert_eq!(counts[0].1, 1);
        assert_eq!(counts[1].1, 1);

        assert_eq!(counts[0].0, "label".to_string());
        assert_eq!(counts[1].0, "label2".to_string());
    }
}
