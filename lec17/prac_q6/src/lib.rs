use std::sync::{RwLock, Arc, atomic::{AtomicUsize, Ordering}};

pub struct RCUType<T> {
    data: Arc<RwLock<Arc<T>>>,
    generation: Arc<AtomicUsize>,
}

impl<T> RCUType<T> {
    /// Creates a new `RCUType` with a given value.
    pub fn new(value: T) -> RCUType<T> {
        todo!()
    }

    /// Will call the closure `updater`, passing the current
    /// value of the type; allowing the user to return a new
    /// value for this to store.
    pub fn update(&self, updater: impl FnOnce(&T) -> T) {
        todo!()
    }

    /// Returns an atomically reference counted smart-pointer
    /// to the most recent copy of data this function has.
    pub fn get(&self) -> Arc<T> {
        todo!()
    }

    /// Return the number of times that the RCUType has been updated.
    pub fn get_generation(&self) -> usize {
        todo!()
    }
}

impl<T> Clone for RCUType<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            generation: self.generation.clone(),
        }
    }
}
