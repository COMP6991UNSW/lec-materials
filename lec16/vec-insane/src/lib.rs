#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::alloc::Layout;

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("ZSTs not supported");
        }

        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.expand_capacity();
        }

        // Safety: if `self.len` was equal to `self.capacity`
        // i.e. `self.len` was out of range,
        // then we would have already expanded the capacity above.
        // Because of this, `self.len` *must be* in-range, so
        // calling `pointer_to_elem` with `self.len` is safe.
        let place = unsafe { self.pointer_to_elem(self.len) };

        // Safety: as above
        unsafe { place.write(value); }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            // Safety: TODO
            let place = unsafe { self.pointer_to_elem(self.len) };

            // Safety: TODO
            Some(unsafe { place.read() })
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // Safety: TODO
            let place = unsafe { self.pointer_to_elem(index) };

            // Safety: TODO
            Some(unsafe { &*place })
        } else {
            None
        }
    }

    fn expand_capacity(&mut self) {
        if self.capacity == 0 {
            const DEFAULT_CAPACITY: usize = 8;

            let layout = Self::layout_array(DEFAULT_CAPACITY);
            // Safety: The layout should describe an array of our Ts
            //         which are larger or equal to one byte each
            //         with a capacity that is greater than zero.
            // TODO: ZSTs
            self.ptr = unsafe { std::alloc::alloc(layout) as _ };
            self.capacity = DEFAULT_CAPACITY;
        } else {
            // Danger: could overflow??
            let new_capacity = self.capacity * 2;

            let old_layout = Self::layout_array(self.capacity);
            let new_layout = Self::layout_array(new_capacity);

            // Safety: self.ptr was allocated with std::alloc::alloc as above
            //         the layout is what was used previously (from Self::layout_array)
            //         new_size is greater than zero (it is doubled, from a non-zero value)
            //         hopefully not too big (TODO)
            self.ptr = unsafe { std::alloc::realloc(self.ptr as _, old_layout, new_layout.size()) as _ };
            self.capacity = new_capacity;
        }
    }

    fn layout_array(capacity: usize) -> Layout {
        Layout::array::<T>(capacity)
            .expect("number too big")
    }

    // # Safety
    // 
    // The index must be in-range for the Vec's capacity
    unsafe fn pointer_to_elem(&self, index: usize) -> *mut T {
        // Safety: Checked by caller with safety docs above
        // TODO: really big numbers might cause UB
        unsafe { self.ptr.add(index) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            for index in 0..self.len {
                // Safety: ...
                unsafe { std::ptr::drop_in_place(self.pointer_to_elem(index)); }
            }

            // Safety: If the ptr is not null, then we allocated memory in
            //         expand capacity in this exact manner as below
            unsafe { std::alloc::dealloc(self.ptr as _, Self::layout_array(self.capacity)); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<String> = MyVec::new();
        vec.push(String::from("Hello"));
        vec.push(String::from("from"));
        vec.push(String::from("cs6991!"));

        for index in 0..4 {
            println!("vec[{index}] = {:?}", vec.get(index));
        }

        println!("{:?}", vec.pop());
        println!("{:?}", vec.pop());
        println!("{:?}", vec.pop());
        println!("{:?}", vec.pop());
        
        for index in 0..4 {
            println!("vec[{index}] = {:?}", vec.get(index));
        }

        println!("I created a vec!!!");
    }

    #[test]
    fn drop_with_no_alloc() {
        let vec: MyVec<String> = MyVec::new();
        drop(vec);
    }
}
