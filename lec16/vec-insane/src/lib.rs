// UNSAFE module
mod my_vec {
    use std::alloc::{Layout, self, realloc};
    
    pub struct MyVec<T> {
        ptr: *mut T,
        size: usize,
        capacity: usize,
    }

    impl<T> MyVec<T> {
        const INITIAL_CAPACITY: usize = 4;

        pub fn new() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                size: 0,
                capacity: 0,
            }
        }

        pub fn push(&mut self, value: T) {
            if self.size == self.capacity {
                // need to allocate!
                self.expand_capacity();
            }

            // Now we're always safe to
            // push one more element!

            // SAFETY: We just expanded the capacity if required,
            // and so self.size will always be a valid index.
            let ptr = unsafe { self.pointer_to_elem(self.size) };

            // SAFETY: The pointer returned by `self.pointer_to_elem`
            // is a valid pointer within our vec allocation,
            // so we're safe to write to this address.
            unsafe { ptr.write(value) };

            self.size += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.size == 0 {
                None
            } else {
                self.size -= 1;

                // SAFETY: We checked that the vec is non-empty,
                // and `self.size` now represents the last valid
                // member of the vec due to the `-= 1` operation.
                let ptr = unsafe { self.pointer_to_elem(self.size) };

                // SAFETY: Same as above, and it is fine to take
                // ownership of the T, because we are effectively
                // *removing* it from the vec by decrementing the size.
                let value = unsafe { ptr.read() };

                Some(value)
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.size {
                None
            } else {
                // SAFETY: We checked that the index provided
                // represents a valid element in the vec.
                let ptr = unsafe { self.pointer_to_elem(index) };

                // SAFETY: Same as above, and it is safe to borrow
                // since it is tied to the lifetime of &self
                let value = unsafe { &*ptr };

                Some(value)
            }
        }

        /// `expand_capacity` will double the
        /// capacity of the `MyVec` if non-empty,
        /// or allocate room for `INITIAL_CAPACITY`
        /// elements if there is currently no
        /// allocation.
        ///
        /// If allocation fails, this function will panic.
        fn expand_capacity(&mut self) {
            if self.capacity == 0 {
                let layout = Self::layout_for(Self::INITIAL_CAPACITY);
                // SAFETY: Layout has non-zero size
                let ptr = unsafe { alloc::alloc(layout) };

                self.ptr = ptr as _;
                self.capacity = Self::INITIAL_CAPACITY;
            } else {
                let new_capacity = self.capacity.checked_mul(2)
                    .expect("capacity overflow");

                let old_layout = Self::layout_for(self.capacity);
                let new_layout = Self::layout_for(new_capacity);

                // SAFETY: We allocated `self.ptr` ourself with a layout equal to `old_layout`,
                //         and the `new_layout` is greater than zero bytes in size,
                //         and we assume it doesn't overflow :3 (unsoundness warning)
                let new_ptr = unsafe { realloc(self.ptr as _, old_layout, new_layout.size()) };

                self.ptr = new_ptr as _;
                self.capacity = new_capacity;
            }
        }

        fn layout_for(n_elems: usize) -> Layout {
            Layout::array::<T>(n_elems)
                    .unwrap()
        }

        /// # Safety
        ///
        /// `index` must be a valid index within our
        /// current ptr
        unsafe fn pointer_to_elem(&self, index: usize) -> *mut T {
            unsafe { self.ptr.add(index) }
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            for index in 0..self.size {
                unsafe { std::ptr::drop_in_place(self.pointer_to_elem(index)); }
            }

            let layout = Self::layout_for(self.capacity);
            
            // SAFETY: We allocated this pointer ourselves,
            // using the layout provided.
            unsafe { alloc::dealloc(self.ptr as _, layout); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::my_vec::MyVec;

    #[test]
    fn it_works() {
        let mut vec = MyVec::new();
        vec.push("hello");
        vec.push("world!");
        vec.push("this");
        vec.push("is");
        vec.push("comp6991!");

        for i in 0..10 {
            println!("{:?}", vec.get(i));
        }

        println!("pop: {:?}", vec.pop());
        println!("pop: {:?}", vec.pop());
        println!("pop: {:?}", vec.pop());

        for i in 0..10 {
            println!("{:?}", vec.get(i));
        }
    }
}
