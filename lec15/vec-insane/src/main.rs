fn main() {
    println!("Hello, world!");
}

mod my_vec {
    use std::alloc::Layout;

    pub struct MyVec<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }

    impl<T> MyVec<T> {
        pub fn new() -> MyVec<T> {
            if std::mem::size_of::<T>() == 0 {
                todo!();
            }
            
            Self {
                ptr: std::ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }

        pub fn push(&mut self, value: T) {
            if self.len == self.cap {
                self.expand_capacity();
            }

            // SAFETY: ...
            let place = unsafe { self.pointer_to_elem(self.len) };

            // SAFETY: ...
            unsafe { place.write(value) };

            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;

                // SAFETY: ...
                let place = unsafe { self.pointer_to_elem(self.len) };

                Some(
                    // SAFETY: ...
                    unsafe { place.read() }
                )
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            todo!()
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            todo!()
        }

        // # Safety
        //
        // The index must be in range (i.e. index < self.len)
        unsafe fn pointer_to_elem(&self, index: usize) -> *mut T {
            // SAFETY: Precondition of this function (safety requirements)
            unsafe { self.ptr.add(index) }
        }

        // double's it
        fn expand_capacity(&mut self) {
            if self.cap == 0 {
                const DEFAULT_CAPACITY: usize = 8;

                let layout = Layout::array::<T>(DEFAULT_CAPACITY)
                    .expect("outside scope");

                // SAFETY: Layout has non zero size
                // (reasoning)
                self.ptr = unsafe { std::alloc::alloc(layout) } as *mut T;
                self.cap = DEFAULT_CAPACITY;
            } else {
                let new_cap = self.cap * 2;

                let curr_layout = Layout::array::<T>(self.cap)
                    .expect("outside scope");
                let new_layout = Layout::array::<T>(new_cap)
                    .expect("outside scope");

                // SAFETY: ...
                self.ptr = unsafe {
                    std::alloc::realloc(
                        self.ptr as *mut u8,
                        curr_layout,
                        new_layout.size()
                    )
                } as *mut T;
                self.cap = new_cap;
            }
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                for index in 0..self.len {
                    // SAFETY: ...
                    unsafe { std::ptr::drop_in_place(self.pointer_to_elem(index)) };
                }

                let curr_layout = Layout::array::<T>(self.cap)
                    .expect("outside scope");

                // SAFETY: ...
                unsafe { std::alloc::dealloc(self.ptr as *mut u8, curr_layout); }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::my_vec::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<String> = MyVec::new();
        vec.push("Hello".to_string());
        vec.push("from".to_string());
        vec.push("cs6991".to_string());

        dbg!(vec.pop());
        dbg!(vec.pop());
        dbg!(vec.pop());
        dbg!(vec.pop());

        assert!(false);
    }
}
