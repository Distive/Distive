use anchorhash::AnchorHash;
// use anchorhash::AnchorHash::
use highway::{HighwayBuildHasher, HighwayHasher};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
pub struct Node<TId: Hash + Eq + Clone, Data> {
    id: TId,
    data: HashMap<String, Data>,
    // all_nodes: HashSet<TId>,
    // next_node_id: Option<TId>,
    // prev_node_id: Option<TId>,
    index_node_id: TId,
    hash: AnchorHash<String, TId, HighwayBuildHasher>,
}

impl<TId: Eq + Hash + Clone, Data> Node<TId, Data> {
    fn new(id: TId, index_node_id: TId, all_nodes: HashSet<TId>) -> Self {
        Node {
            id,
            index_node_id,
            hash: anchorhash::Builder::with_hasher(Default::default())
                .with_resources(all_nodes)
                .build(100),
            data: HashMap::new(),
        }
    }

    fn with_data_mut<'a, F, R>(&mut self, key: String, action: F) -> NodeResult<TId, Option<R>>
    where
        F: FnOnce(&mut Data) -> R,
    {
        match self.node_id_from_data_key(key.clone()) {
            Some(node_id) => {
                if node_id.clone() == self.id {
                    match self.data.get_mut(&key) {
                        Some(data) => NodeResult::Result(Some(action(data))),
                        None => NodeResult::Result(None),
                    }
                } else {
                    NodeResult::NodeId(node_id.clone())
                }
            }
            None => NodeResult::Result(None),
        }
    }

    // fn handle_request(request: Request) -> Response {}
    // fn ping() {}
    fn node_id_from_data_key(&self, data_key: String) -> Option<&TId> {
        self.hash.get_resource(data_key)
    }

    fn add_node(&mut self, node_id: TId) {
        self.hash.add_resource(node_id);
    }

    fn remove_node(&mut self, node_id: &TId) {
        self.hash.remove_resource(node_id);
    }

    
    // fn migrate_data_request(node_id: TId, data: HashMap<String, Data>) -> Response {}
}

#[derive(Debug, PartialEq)]
pub enum NodeResult<TId, Data> {
    NodeId(TId),
    Result(Data),
}

impl<TId, Data> NodeResult<TId, Data> {
    fn or_forward_unwrap<O: FnOnce(TId) -> Data>(self, op: O) -> Data {
        match self {
            NodeResult::NodeId(node_id) => op(node_id),
            NodeResult::Result(data) => data,
        }
    }
}

enum Request {
    Migrate,
    Ping,
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_node() {
        let mut node_1 = Node::<_, String>::new(
            "node_1".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        node_1.add_node("node_1".to_string());
        assert_eq!(
            node_1.node_id_from_data_key("data_key".to_string()),
            Some(&"node_1".to_string())
        );
    }

    #[test]
    fn can_remove_node() {
        let mut node_1 = Node::<_, String>::new(
            "node_1".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        node_1.add_node("node_1".to_string());
        node_1.remove_node(&"node_1".to_string());
        assert_eq!(node_1.node_id_from_data_key("data_key".to_string()), None);
    }

    #[test]
    fn data_is_distributed_on_different_nodes() {
        let mut index_node = Node::<_, String>::new(
            "index_node_id".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        index_node.add_node("index_node_id".to_string());
        index_node.add_node("node_1".to_string());

        let mut node_ids: Vec<String> = vec![];

        for id in 1..15 {
            let data_key = format!("data_key_{}", id);
            let node_id = index_node.node_id_from_data_key(data_key).unwrap();
            node_ids.push(node_id.clone());
        }

        // "index_node_id" and "node_1" should exist in node_ids
        assert!(node_ids.contains(&"node_1".to_string()));
        assert!(node_ids.contains(&"index_node_id".to_string()));

        //node_1 should exist atleast 4 times in node_ids, Iterator::filter()
        let node_1_count = node_ids
            .iter()
            .filter(|&x| x == &"node_1".to_string())
            .count();
        assert!(node_1_count >= 4);

        //same with index_node_id
        let index_node_id_count = node_ids
            .iter()
            .filter(|&x| x == &"index_node_id".to_string())
            .count();
        assert!(index_node_id_count >= 4);
    }
    #[test]
    fn node_id_from_data_key_must_be_deterministic() {
        let mut node_ids: Vec<String> = vec![];

        for _ in 1..100 {
            let mut index_node = Node::<_, String>::new(
                "index_node_id".to_string(),
                "index_node_id".to_string(),
                HashSet::new(),
            );

            index_node.add_node("index_node_id".to_string());
            index_node.add_node("node_1".to_string());

            let data_key = "data_key".to_string();
            let node_id = index_node.node_id_from_data_key(data_key).unwrap();
            node_ids.push(node_id.clone());
        }
        //all ids should be same
        assert_eq!(node_ids.iter().collect::<HashSet<_>>().len(), 1);
    }

    #[test]
    fn with_data_mut_returns_result_none_when_key_maps_to_node_but_data_not_available() {
        let mut node_1 = Node::<_, String>::new(
            "index_node_id".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        node_1.add_node("index_node_id".to_string());

        let result = node_1.with_data_mut("data_key".to_string(), |data| {
            data.push_str("data");
            data.clone()
        });

        assert_eq!(result, NodeResult::Result(None));
    }

    #[test]
    fn with_data_mut_returns_result_when_key_maps_to_node_and_data_available() {
        let mut node_1 = Node::<_, String>::new(
            "index_node_id".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        node_1.add_node("index_node_id".to_string());
        node_1
            .data
            .insert("data_key".to_string(), "data".to_string());

        let result = node_1.with_data_mut("data_key".to_string(), |data| data.clone());

        assert_eq!(result, NodeResult::Result(Some("data".to_string())));
    }

    #[test]
    fn with_data_mut_returns_node_id_when_key_maps_to_different_node() {
        let mut node_1 = Node::<_, String>::new(
            "index_node_id".to_string(),
            "index_node_id".to_string(),
            HashSet::new(),
        );

        node_1.add_node("index_node_id".to_string());
        node_1.add_node("node_1".to_string());

        node_1
            .data
            .insert("data_key".to_string(), "data".to_string());
        node_1
            .data
            .insert("data_key_2".to_string(), "data_2".to_string());

        let result = node_1.with_data_mut("data_key_5".to_string(), |data| data.clone());

        assert_eq!(result, NodeResult::NodeId("node_1".to_string()));
    }
}
