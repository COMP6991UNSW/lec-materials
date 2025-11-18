#![allow(unused)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::alloc::{self, Layout};

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    const FIRST_CAPACITY: usize = 4;

    pub fn new() -> MyVec<T> {
        if std::mem::size_of::<T>() == 0 {
            panic!("ZSTs are unsupported");
        }

        MyVec {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len >= self.cap {
            self.expand_capacity();
        }

        // We definitely have room for at least 1 more element
        // self.len < self.cap

        let place = self.ptr.wrapping_add(self.len);

        // SAFETY: The ptr we derive is correct and in the vec's
        // allocation space, so we can set its contents to the T
        unsafe { place.write(value); }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let place = self.ptr.wrapping_add(self.len - 1);
        // SAFETY:
        // 1. src must be valid for reads.
        //    ....
        // 2. src must be properly aligned.
        //    ....
        // 3.
        let value = unsafe { place.read() };
        self.len -= 1;

        Some(value)
    }

    pub fn len(&self) -> usize { self.len }

    fn expand_capacity(&mut self) {
        if self.cap == 0 {
            // No existing allocation for data
            let layout = Layout::array::<T>(Self::FIRST_CAPACITY)
                .unwrap();

            // SAFETY:
            // In MyVec::new, we check that T is not a ZST,
            // and if it is, we panic s.t. the user does not get
            // a MyVec to use.
            // Since T is not a ZST, it is at least 1 byte,
            // which if we allocate 4 of, will be at least 4 bytes.
            let ptr = unsafe { alloc::alloc(layout) };

            self.ptr = ptr as _;
            self.cap = Self::FIRST_CAPACITY;
        } else {
            // Existing allocation has been exhausted

            let new_capacity = self.cap * 2;

            let old_layout = Layout::array::<T>(self.cap)
                .unwrap();
            let new_layout = Layout::array::<T>(new_capacity)
                .unwrap();

            // SAFETY:
            // Left as an exercise to the reader!
            let ptr = unsafe { alloc::realloc(
                self.ptr as _,
                old_layout,
                new_layout.size(),
            ) };

            self.ptr = ptr as _;
            self.cap = new_capacity;
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }

        // while let Some(value) = self.pop() {}
        for index in 0..self.len {
            let ptr = self.ptr.wrapping_add(index);

            // SAFETY: ...
            unsafe { std::ptr::drop_in_place(ptr); }
        }

        let layout = Layout::array::<T>(self.cap)
            .unwrap();
        // SAFETY: ...
        unsafe { alloc::dealloc(self.ptr as _, layout); }
    }
}

fn main() {
    let x = 42;
    let x_ptr = &x as *const _;
    let x_ptr_2 = x_ptr as *mut i32;
    let x_ptr_2: *mut String = x_ptr as _;

    let mut my_vec = MyVec::<i32>::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    dbg!(my_vec.pop());
    dbg!(my_vec.pop());
    dbg!(my_vec.pop());
    dbg!(my_vec.pop());

    let mut my_2nd_vec = MyVec::<i32>::new();

    let mut v = MyVec::new();
    v.push(String::from("hello"));
    v.push(String::from("world"));
    v.push(String::from("its"));
    v.push(String::from("zac"));
    dbg!(v.pop());
    dbg!(v.pop());
}
