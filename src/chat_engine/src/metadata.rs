use hashbrown::HashMap;
use hashbrown::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Default, Clone, Debug)]
pub struct Metadata {
    pub value: HashMap<String, HashSet<String>>,
}

pub struct MetadataInput {
    pub label: String,
    pub user_id: String,
}

/// (label, number of users, toggled by user_ids)
pub type MetadataOutput = Vec<(String, usize, Vec<bool>)>;

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

    fn get_toggled_users_bool(
        metadata_users: &HashSet<String>,
        user_ids: &Vec<String>,
    ) -> Vec<bool> {
        user_ids
            .iter()
            .map(|user_id| metadata_users.contains(user_id))
            .collect()
    }

    pub fn to_output(&self, user_ids: &Vec<String>) -> MetadataOutput {
        self.value
            .iter()
            .map(|(label, users)| {
                (
                    label.to_string(),
                    users.len(),
                    Self::get_toggled_users_bool(users, user_ids),
                )
            })
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
    use rand::Rng;

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
        let metadata_output = metadata.to_output(&vec!["user_id".to_string()]);

        let (label, user_count, is_toggled) = metadata_output[0].clone();

        assert_eq!(
            (label, user_count, is_toggled),
            ("label".to_string(), 1, vec![true])
        );

        metadata.toggle_value(&"user_id".to_string(), &"label".to_string());
        let metadata_output = metadata.to_output(&vec!["user_id".to_string()]);

        let (label, user_count, is_toggled) = metadata_output[0].clone();

        assert_eq!(
            (label, user_count, is_toggled),
            ("label".to_string(), 0, vec![false])
        );
    }

    #[test]
    fn get_toggled_users_bool_is_correct() {
        let metadata_users = HashSet::from_iter(vec!["user_id".to_string()]);
        let mut user_ids = vec!["user_id".to_string()];
        let toggled_users_bool = Metadata::get_toggled_users_bool(&metadata_users, &user_ids);
        assert_eq!(toggled_users_bool, vec![true]);

        // returns same length as user_ids
        // generate a random number of users and check that the length is the same as user_ids
        let mut rng = rand::thread_rng();
        let user_count = rng.gen_range(0, 10);
        let mut metadata_users = HashSet::new();
        for _ in 0..user_count {
            // metadata_users.insert(rng.gen_range(0, 10).to_string());
            let user_id = rng.gen_range(0, 10).to_string();
            metadata_users.insert(user_id.clone());
            user_ids.push(user_id);
        }
        let toggled_users_bool = Metadata::get_toggled_users_bool(&metadata_users, &user_ids);
        assert_eq!(toggled_users_bool.len(), user_ids.len());
        assert_eq!(false, toggled_users_bool[0]); //for the first user_id, it is not toggled
    }
}
