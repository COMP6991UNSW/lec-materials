use std::{ptr, ops::Deref};

pub struct Lazy<T> {
    value: *mut T,
    f: fn() -> T,
}

impl<T> Lazy<T> {
    pub fn new(f: fn() -> T) -> Lazy<T> {
        Self {
            value: ptr::null_mut(),
            f,
        }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // if the value is uninitialized, then initialize it!
            if self.value.is_null() {
                let value = (self.f)();
                self.value.write(value);
            }

            // the value is initialized, give out a borrow
            &*self.value
        }
    }
}
