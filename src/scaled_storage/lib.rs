use anchorhash::AnchorHash;
// use anchorhash::AnchorHash::
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
    hash: AnchorHash<String, TId, RandomState>,
}

impl<TId: Eq + Hash + Clone, Data> Node<TId, Data> {
    fn new(id: TId, index_node_id: TId, all_nodes: HashSet<TId>) -> Self {
        Node {
            id,
            index_node_id,
            hash: anchorhash::Builder::default()
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

// type RedirectParams = (Principal,Args)
enum Request {
    Migrate,
    Ping,
}

struct Response {}

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
        // let d = node_1
        //     .with_data_mut("test".to_string(), |data| "".to_string())
        //     .or_forward_unwrap(|node_id| Some("".to_string()));
    }
}
