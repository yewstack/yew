use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::collections::{hash_map, HashMap, HashSet};

/// Thread-local instance used to queue worker messages
pub struct Queue {
    loaded_agents: RefCell<HashSet<TypeId>>,
    msg_queue: RefCell<HashMap<TypeId, Vec<Vec<u8>>>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            loaded_agents: RefCell::new(HashSet::new()),
            msg_queue: RefCell::new(HashMap::new()),
        }
    }

    #[inline]
    pub fn borrow_msg_queue_mut(&self) -> RefMut<'_, HashMap<TypeId, Vec<Vec<u8>>>> {
        self.msg_queue.borrow_mut()
    }

    #[inline]
    pub fn insert_loaded_agent(&self, type_id: TypeId) {
        self.loaded_agents.borrow_mut().insert(type_id);
    }

    #[inline]
    pub fn is_worker_loaded(&self, type_id: &TypeId) -> bool {
        self.loaded_agents.borrow().contains(type_id)
    }

    pub fn add_msg_to_queue(&self, msg: Vec<u8>, type_id: TypeId) {
        let mut queue = self.msg_queue.borrow_mut();
        match queue.entry(type_id) {
            hash_map::Entry::Vacant(record) => {
                record.insert(vec![msg]);
            }
            hash_map::Entry::Occupied(ref mut record) => {
                record.get_mut().push(msg);
            }
        }
    }

    /// This is called by a worker's `Drop` implementation in order to remove the worker from the list
    /// of loaded workers.
    pub fn remove_agent(&self, type_id: &TypeId) {
        self.loaded_agents.borrow_mut().remove(type_id);
        self.msg_queue.borrow_mut().remove(type_id);
    }
}
