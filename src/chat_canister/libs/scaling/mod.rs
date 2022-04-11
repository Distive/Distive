use mopa::*;
use std::collections::HashSet;

pub trait ScaledData: Any {
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
}
mopafy!(ScaledData);

pub struct ScaledStorage {
    pub id: String,
    data: Box<dyn ScaledData>,
    sibling_canister_ids: HashSet<String>,
    next_canister_id: Option<String>,
    prev_canister_id: Option<String>,
}

impl ScaledStorage {
    pub fn new<T>(
        id: String,
        sibling_canister_ids: HashSet<String>,
        prev_canister_id: Option<String>,
        data: T,
    ) -> Self
    where
        T: ScaledData + 'static,
    {
        ScaledStorage {
            id,
            data: Box::new(data),
            sibling_canister_ids,
            prev_canister_id,
            next_canister_id: None,
        }
    }

    pub fn get_data<T: ScaledData>(&self) -> &T {
        self.data.downcast_ref::<T>().unwrap()
    }

    fn get_data_mut<T: ScaledData>(&mut self) -> &mut T {
        self.data.downcast_mut::<T>().unwrap()
    }

    pub fn with_data_mut<T: ScaledData, E, F, R>(&mut self, action: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        action(self.get_data_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ListU32Data {
        data: Vec<u32>,
    }

    impl ScaledData for ListU32Data {
        fn is_full(&self) -> bool {
            self.data.len() > 2
        }

        fn is_empty(&self) -> bool {
            self.data.len() == 0
        }
    }

    #[test]
    fn correct_properties_initialized() {
        let id = "id".to_string();
        let mut sibling_canister_ids = HashSet::new();
        sibling_canister_ids.insert("canister_id_1".to_string());
        let prev_canister_id = None;
        let data = ListU32Data {
            data: vec![1, 2, 3],
        };
        let storage = ScaledStorage::new(
            id.clone(),
            sibling_canister_ids.clone(),
            prev_canister_id.clone(),
            data,
        );
        assert_eq!(storage.id, id);
        assert_eq!(storage.sibling_canister_ids, sibling_canister_ids);
        assert_eq!(storage.prev_canister_id, prev_canister_id);
        assert_eq!(storage.next_canister_id, None);
        assert_eq!(storage.get_data::<ListU32Data>().data, vec![1, 2, 3]);
    }

    #[test]
    fn get_data_should_return_implementation() {
        let data = ListU32Data { data: vec![1] };

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), None, data);

        let list_u32_scaled_data = scaled_storage.get_data::<ListU32Data>();
        assert_eq!(list_u32_scaled_data.data[0], 1);
    }

    #[test]
    fn get_data_mut_should_return_implementation_and_be_mutable() {
        let data = ListU32Data { data: vec![1] };

        let mut scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), None, data);

        let list_u32_scaled_data = scaled_storage.get_data_mut::<ListU32Data>();
        assert_eq!(list_u32_scaled_data.data[0], 1);

        list_u32_scaled_data.data.push(2);
        assert_eq!(list_u32_scaled_data.data[1], 2);
    }
    #[test]
    fn is_full_should_be_correct() {
        let data = ListU32Data {
            data: vec![1, 2, 3],
        };

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), None, data);

        assert!(scaled_storage.get_data::<ListU32Data>().is_full());
    }

    #[test]
    fn is_empty_should_be_correct() {
        let data = ListU32Data { data: vec![] };

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), None, data);

        assert!(scaled_storage.get_data::<ListU32Data>().is_empty());
    }

    #[test]
    fn with_data_mut_should_take_a_closure_and_be_mutable() {
        let data = ListU32Data { data: vec![1] };

        let mut scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), None, data);

        let result = scaled_storage
            .with_data_mut::<ListU32Data, String, _, Result<String, String>>(
                |list_u32_scaled_data| {
                    list_u32_scaled_data.data.push(2);
                    list_u32_scaled_data.data.push(3);
                    Ok("OK".to_string())
                },
            );

        assert_eq!(result.unwrap(), "OK");
        assert_eq!(scaled_storage.get_data::<ListU32Data>().data, vec![1, 2, 3]);
    }
}
