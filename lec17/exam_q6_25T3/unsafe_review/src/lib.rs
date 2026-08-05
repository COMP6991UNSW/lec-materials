use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn replace(&self, value: T) -> T {
        unsafe {
            std::mem::replace(&mut *self.value.get(), value)
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}
