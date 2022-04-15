use ic_cdk;
use ic_cdk::call;

use ic_cdk::export::candid::utils::ArgumentDecoder;
use mopa::*;
use std::collections::HashSet;
// use ic_cdk::export::{Principal};
pub trait ScaledData: Any {
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    // fn get_canister_id(&self) -> Option<String>;
    // fn set_canister_id(&self, canister_id: String) -> ();
}

mopafy!(ScaledData);

pub struct ScaledStorage {
    pub id: String,
    data: Box<dyn ScaledData>,
    sibling_canister_ids: HashSet<String>,
    canister_id: String,
}

impl ScaledStorage {
    pub fn new<T>(id: String, sibling_canister_ids: HashSet<String>, data: T) -> Self
    where
        T: ScaledData + 'static,
    {
        ScaledStorage {
            id,
            data: Box::new(data),
            sibling_canister_ids,
            canister_id: ic_cdk::id().to_text(),
        }
    }

    pub fn get_data<T: ScaledData>(&self) -> &T {
        self.data.downcast_ref::<T>().unwrap()
    }

    fn get_data_mut<T: ScaledData>(&mut self) -> &mut T {
        self.data.downcast_mut::<T>().unwrap()
    }

    
    pub fn with_data_mut<'a, T: ScaledData, F, R, C>(
        &mut self,
        action: F,
        canister_id: Option<String>,
       
    ) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        match canister_id {
            Some(canister_id) => {
                if canister_id == self.canister_id {
                    Some(action(self.get_data_mut()))
                } else {
                    None
                }
            }
            None => Some(action(self.get_data_mut())),
        }
    }

    // fn create_canister(&mut self, id: String) -> ScaledStorage {
    //     let canister = ScaledStorage::new(
    //         id,
    //         HashSet::new(),
    //         self.next_canister_id.take(),
    //         ScaledStorage::default(),
    //     );
    //     self.next_canister_id = Some(id);
    //     canister
    // }
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

    

        // fn get_canister_id(&self) -> Option<String> {
        //     None
        // }

        // fn set_canister_id(&self, _id: String) {}
    }

    #[test]
    fn correct_properties_initialized() {
        let id = "id".to_string();
        let mut sibling_canister_ids = HashSet::new();
        sibling_canister_ids.insert("canister_id_1".to_string());
        let data = ListU32Data {
            data: vec![1, 2, 3],
        };
        let storage = ScaledStorage::new(id.clone(), sibling_canister_ids.clone(), data);
        assert_eq!(storage.id, id);
        assert_eq!(storage.sibling_canister_ids, sibling_canister_ids);
        assert_eq!(storage.get_data::<ListU32Data>().data, vec![1, 2, 3]);
    }

    #[test]
    fn get_data_should_return_implementation() {
        let data = ListU32Data { data: vec![1] };

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), data);

        let list_u32_scaled_data = scaled_storage.get_data::<ListU32Data>();
        assert_eq!(list_u32_scaled_data.data[0], 1);
    }

    #[test]
    fn get_data_mut_should_return_implementation_and_be_mutable() {
        let data = ListU32Data { data: vec![1] };

        let mut scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), data);

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

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), data);

        assert!(scaled_storage.get_data::<ListU32Data>().is_full());
    }

    #[test]
    fn is_empty_should_be_correct() {
        let data = ListU32Data { data: vec![] };

        let scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), data);

        assert!(scaled_storage.get_data::<ListU32Data>().is_empty());
    }

    #[test]
    fn with_data_mut_should_take_a_closure_and_be_mutable() {
        let data = ListU32Data { data: vec![1] };
        let mut scaled_storage = ScaledStorage::new("id".to_string(), HashSet::new(), data);
        // let result = scaled_storage.with_data_mut::<ListU32Data, _, _>(
        //     |list_u32_scaled_data| {
        //         list_u32_scaled_data.data.push(2);
        //         list_u32_scaled_data.data.push(3);
        //         Ok("OK".to_string()) as Result<String, String>
        //     },
        //     None,
        // );

        // assert_eq!(result.unwrap(), "OK");
        // assert_eq!(scaled_storage.get_data::<ListU32Data>().data, vec![1, 2, 3]);
    }

    // #[test]
    //if_full
}
