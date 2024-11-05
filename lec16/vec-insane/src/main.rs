use my_vec::MyVec;

mod my_vec {
    use std::{alloc::{self, Layout}, ptr};

    pub struct MyVec<T> {
        ptr: *mut T,
        len: usize, // how many elements are we currently storing
        cap: usize, // how many elements are we currently *able* to store
    }

    const INITIAL_CAPACITY: usize = 4;

    impl<T> MyVec<T> {
        pub fn new() -> Self {
            if std::mem::size_of::<T>() == 0 {
                panic!("not today kiddo");
            }

            Self {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }

        pub fn push(&mut self, value: T) {
            if self.len == self.cap {
                self.expand_capacity();
            }

            let ptr = self.pointer_to_elem(self.len);
            // Safety: This is ok because we know from
            // the capacity check that there is memory we
            // have allocated to place this value into
            unsafe { ptr.write(value); }
            self.len += 1;
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                return None;
            }

            let ptr = self.pointer_to_elem(index);
            // Safety: ...
            Some(unsafe { &*ptr })
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }

            self.len -= 1;
            let ptr = self.pointer_to_elem(self.len);
            // Safety: ...
            let value = unsafe { ptr.read() };

            Some(value)
        }

        fn pointer_to_elem(&self, index: usize) -> *mut T {
            self.ptr.wrapping_add(index)
        }

        fn expand_capacity(&mut self) {
            if self.cap == 0 {
                // We need to allocate INITIAL_CAPACITY size

                let layout = Layout::array::<T>(INITIAL_CAPACITY)
                    .expect("out of scope");

                // Safety: We make sure ZSTs are rejected on Self::new
                let allocation = unsafe { alloc::alloc(layout) };

                self.ptr = allocation as *mut _;
                self.cap = INITIAL_CAPACITY;
            } else {
                // We need to double the capacity of our current allocation

                let new_capacity = self.cap * 2;

                let curr_layout = Layout::array::<T>(self.cap)
                    .expect("out of scope");
                let new_layout = Layout::array::<T>(new_capacity)
                    .expect("out of scope");

                let allocation = unsafe { alloc::realloc(
                        self.ptr as *mut _,
                        curr_layout,
                        new_layout.size())
                };

                self.ptr = allocation as *mut _;
                self.cap = new_capacity;
            }
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            if self.ptr.is_null() {
                return;
            }

            for index in 0..self.len {
                let ptr = self.pointer_to_elem(index);
                // Safety: ...
                unsafe { ptr::drop_in_place(ptr) };
            }

            let layout = Layout::array::<T>(self.cap)
                .expect("out of scope");

            unsafe { alloc::dealloc(self.ptr as _, layout); }
        }
    }
}

fn main() {
    let mut v = MyVec::new();
    v.push(String::from("42"));
    v.push(String::from("42"));
    v.push(String::from("42"));
    v.push(String::from("42"));
    v.push(String::from("42"));

    dbg!(v.get(100));
    dbg!(v.get(0));
    dbg!(v.get(0));
    dbg!(v.pop());
    dbg!(v.pop());
}
