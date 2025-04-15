#![allow(unused)]

use my_vec::MyVec;

mod my_vec {
    use std::{alloc::{self, Layout}, ptr::{self, drop_in_place}};

    pub struct MyVec<T> {
        buf: *mut T,
        len: usize,
        cap: usize,
    }

    impl<T> MyVec<T> {
        const INITIAL_CAPACITY: usize = 4;

        pub fn new() -> Self {
            // This check is relied upon by unsafe code elsewhere
            let size = std::mem::size_of::<T>();
            if size == 0 {
                panic!("ZSTs are not supported");
            }

            Self {
                buf: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }

        pub fn push(&mut self, value: T) {
            if self.len == self.cap {
                self.expand_capacity();
            }

            // self.buf is valid!
            // self.len < self.cap
            let first_ptr = self.buf;
            let push_ptr = first_ptr.wrapping_add(self.len);

            // SAFETY:
            // This is a valid location to store a value!
            unsafe { push_ptr.write(value); }
            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }

            let first_ptr = self.buf;
            let last_ptr = first_ptr.wrapping_add(self.len - 1);

            // SAFETY:
            // This is a valid location to read a value from!
            let value = unsafe { last_ptr.read() };
            self.len -= 1;

            Some(value)
        }

        fn expand_capacity(&mut self) {
            if self.buf.is_null() {
                // make an initial allocation
                // self.buf = malloc(sizeof(T) * INITIAL_CAPACITY);
                let layout = Layout::array::<T>(Self::INITIAL_CAPACITY)
                    .expect("the allocation is too big");

                // SAFETY:
                // > layout must have non-zero size.
                // > Attempting to allocate for a zero-sized
                // > layout may result in undefined behavior.
                //
                // The Layout can only be zero sized if T is ZST,
                // and that is rejected in Self::new with a panic.
                self.buf = unsafe { alloc::alloc(layout) } as _;
                self.cap = Self::INITIAL_CAPACITY;
            } else {
                // expand the existing allocation
                let old_cap = self.cap;
                let new_cap = 2 * old_cap;

                let old_layout = Layout::array::<T>(old_cap)
                    .expect("the allocation is too big");
                let new_layout = Layout::array::<T>(new_cap)
                    .expect("the allocation is too big");

                // SAFETY:
                // > ptr is allocated via this allocator,
                // => the ptr came from a previous alloc::alloc or alloc::realloc, so this is true!
                // > layout is the same layout that was used to allocate that block of memory,
                // => we use the existing capacity which is tied to the last allocation, so this is true!
                // > new_size is greater than zero.
                // => See note above about Layout ZST 
                // > new_size, when rounded up to the nearest multiple of layout.align(), does not overflow isize (i.e., the rounded value must be less than or equal to isize::MAX).
                // => pretty sure!
                self.buf = unsafe { alloc::realloc(self.buf as _, old_layout, new_layout.size()) } as _;
                self.cap = new_cap;
            }
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            if self.buf.is_null() {
                return;
            }

            for index in 0..self.len {
                let ptr = self.buf.wrapping_add(index);
                // SAFETY: bla bla bla
                unsafe { drop_in_place(ptr) };
            }

            let layout = Layout::array::<T>(self.cap)
                .expect("the allocation is too big");
            // SAFETY: bla bla bla
            unsafe { alloc::dealloc(self.buf as _, layout) };
        }
    }
}

fn main() {
    let mut my_vec = MyVec::new();
    my_vec.push(String::from("1"));
    my_vec.push(String::from("2"));
    my_vec.push(String::from("3"));
    // dbg!(my_vec.pop());
    // dbg!(my_vec.pop());
    // dbg!(my_vec.pop());
    dbg!(my_vec.pop());
}
