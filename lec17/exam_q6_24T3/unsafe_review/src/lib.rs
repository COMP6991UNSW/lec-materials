use std::{ptr, ops::Deref};

pub struct Lazy<T> {
    value: *mut Option<T>,
    f: fn() -> T,
}

impl<T> Lazy<T> {
    pub fn new(f: fn() -> T) -> Lazy<T> {
        let v = Box::new(None::<T>);

        Self {
            value: Box::into_raw(v),
            f,
        }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // if the value is uninitialized, then initialize it!
            let value = self.value.read();
            if value.is_none() {
                let value = (self.f)();
                self.value.write(Some(value));
            }
            std::mem::forget(value);

            // the value is initialized, give out a borrow
            (*self.value).as_ref().unwrap()
        }
    }
}
