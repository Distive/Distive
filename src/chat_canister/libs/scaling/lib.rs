use std::collections::HashSet;

// pub trait ScaledStorage<T> {
//     is_full(&self) -> bool;
//     get_data(&self) -> T;
//     update_data(&mut self);
// }

struct ScaledStorage<T> {
    id: String
    data: T
    sibling_canister_ids: HashSet<String>
    next_canister_id: Option<String>
    prev_canister_id: Option<String>
    status: CanisterStatus
 }

 enum CanisterStatus {
   
 }


impl Canister<T> {
    fn new(sibling_canister_ids: HashSet<String>, data: T) -> Self {
        Self {
            id,
            data,
            sibling_canister_ids,
        }
        // update sibling canisters
    }

    fn is_full(&self) -> bool {
        false
    }

    fn destroy(&mut self) {
        // self.status = CanisterStatus::Destroyed;
        // call previous canister and migrate all data to it
        // uninstall self and update sibling canisters
    }


   pub fn update_data()->{
        //TODO
        // if is full, then forward updated data to next canister
        // if not full, then update data on current canister
    }

    pub fn get_data()->{
        //TODO
        //attach canister id component to data
    }

   
   
}

// tests 
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn _ {

    }
}