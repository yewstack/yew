use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::collections::{hash_map, HashMap, HashSet};

pub struct Queue {
    loaded_agents: RefCell<HashSet<TypeId>>,
    msgs_queue: RefCell<HashMap<TypeId, Vec<Vec<u8>>>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            loaded_agents: RefCell::new(HashSet::new()),
            msgs_queue: RefCell::new(HashMap::new()),
        }
    }

    pub fn borrow_msg_queue_mut(&self) -> RefMut<'_, HashMap<TypeId, Vec<Vec<u8>>>> {
        self.msgs_queue.borrow_mut()
    }

    pub fn insert_loaded(&self, type_id: TypeId) {
        self.loaded_agents.borrow_mut().insert(type_id);
    }

    pub fn is_worker_loaded(&self, type_id: &TypeId) -> bool {
        self.loaded_agents.borrow().contains(type_id)
    }

    pub fn msg_to_queue(&self, msg: Vec<u8>, type_id: TypeId) {
        let mut queue = self.msgs_queue.borrow_mut();
        match queue.entry(type_id) {
            hash_map::Entry::Vacant(record) => {
                record.insert(vec![msg]);
            }
            hash_map::Entry::Occupied(ref mut record) => {
                record.get_mut().push(msg);
            }
        }
    }

    pub fn remove_from_queue(&self, type_id: &TypeId) {
        self.loaded_agents.borrow_mut().remove(type_id);
        self.msgs_queue.borrow_mut().remove(type_id);
    }
}
