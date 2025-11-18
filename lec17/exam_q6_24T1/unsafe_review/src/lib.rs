use std::ptr::{self, addr_of, addr_of_mut};
use std::ops::{Deref, DerefMut};

pub struct Rc<T> {
    ptr: *mut RcBox<T>,
}

struct RcBox<T> {
    count: usize,
    value: T,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcBox {
            count: 1,
            value
        });

        Self {
            ptr: Box::into_raw(inner),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let count_addr = unsafe { addr_of_mut!((*self.ptr).count) };
        unsafe { *count_addr += 1 };

        Self {
            ptr: self.ptr,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let addr = unsafe { addr_of!((*self.ptr).value) };
        unsafe { &*addr }
   }
}

impl<T> DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let addr = unsafe { addr_of_mut!((*self.ptr).value) };
        unsafe { &mut *addr }
   }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let count_addr = addr_of_mut!((*self.ptr).count);
            let count = ptr::read(count_addr);
            let new_count = count - 1;

            if new_count > 0 {
                ptr::write(count_addr, new_count);
            } else {
                drop(Box::from_raw(self.ptr))
            }
        }
    }
}
