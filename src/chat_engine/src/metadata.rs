use hashbrown::HashMap;
use hashbrown::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Default, Clone)]
pub struct Metadata {
    pub value: HashMap<String, HashSet<String>>,
}

pub struct MetadataInput {
    pub label: String,
    pub user_id: String,
}

/// (label, number of users, toggled by current_user)
pub type MetadataOutput = Vec<(String, usize, bool)>;

impl Metadata {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

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

    pub fn to_output(&self, user_id: &String) -> MetadataOutput {
        self.value
            .iter()
            .map(|(label, users)| (label.to_string(), users.len(), users.contains(user_id)))
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
    fn metadata_output_is_correct() {
        let mut metadata = Metadata::default();
        metadata.toggle_value(&"user_id".to_string(), &"label".to_string());
        let metadata_output = metadata.to_output(&"user_id".to_string());

        let (label, user_count, is_toggled) = metadata_output[0].clone();

        assert_eq!(
            (label, user_count, is_toggled),
            ("label".to_string(), 1, true)
        );

        metadata.toggle_value(&"user_id".to_string(), &"label".to_string());
        let metadata_output = metadata.to_output(&"user_id".to_string());

        let (label, user_count, is_toggled) = metadata_output[0].clone();

        assert_eq!(
            (label, user_count, is_toggled),
            ("label".to_string(), 0, false)
        );
    }
}
