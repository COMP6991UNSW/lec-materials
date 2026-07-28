use std::{alloc::{Layout, alloc, dealloc, realloc}, ptr::{self, drop_in_place}};

struct MyVec<T> {
    ptr: *mut T, // points to the first elem, the rest sit next to that
    len: usize, // how many elements do we currently hold
    cap: usize, // how many elements do we have room for
}

impl<T> MyVec<T> {
    fn new() -> Self {
        if size_of::<T>() == 0 {
            panic!("ZSTs are unsupported");
        }

        Self {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len >= self.cap {
            // there's no room left!
            self.expand_capacity();
        }

        assert!(self.len < self.cap);
        let ptr_first = self.ptr;
        let ptr_next = ptr_first.wrapping_add(self.len);

        // SAFETY: ...
        unsafe {
            ptr_next.write(value);
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let ptr_first = self.ptr;
        let ptr_last = ptr_first.wrapping_add(self.len - 1);

        // SAFETY:
        // This location is valid to read a T from.
        let value = unsafe { ptr_last.read() };
        self.len -= 1;

        Some(value)
    }

    /// If self.cap == 0, then this will allocate room for
    /// INITIAL_CAPACITY elements in the vec.
    /// 
    /// Otherwise, it will double self.cap.
    fn expand_capacity(&mut self) {
        // This must be non-zero
        const INITIAL_CAPACITY: usize = 4;

        if self.cap == 0 {
            let layout = Layout::array::<T>(INITIAL_CAPACITY)
                .expect("out of scope!");

            // SAFETY:
            // Layout must have non-zero size: INITIAL_CAPACITY is non-zero as above.
            // And, ZSTs are unsupported and gated by a panic in `Self::new`.
            let ptr = unsafe { alloc(layout) };

            self.ptr = ptr as *mut T;
            self.cap = INITIAL_CAPACITY;
        } else {
            let old_cap = self.cap;
            let new_cap = 2 * old_cap;

            let old_layout = Layout::array::<T>(old_cap)
                .expect("out of scope!");
            let new_layout = Layout::array::<T>(new_cap)
                .expect("out of scope!");

            // SAFETY:
            // The caller must ensure that:
            // * ptr is allocated via this allocator:
            //     we allocated this with either alloc (above) or a previous realloc (below).
            //
            // * layout is the same layout that was used to allocate that block of memory,
            //     we construct it the same way as previously.
            //
            // * new_size is greater than zero.
            //     our old layouts are always non-zero, and the new layout is double that.
            //
            // * new_size, when rounded up to the nearest multiple of layout.align(),
            // does not overflow isize (i.e., the rounded value must be less than or
            // equal to isize::MAX).
            //      out of scope.
            let ptr = unsafe { 
                realloc(self.ptr as _, old_layout, new_layout.size())
            };

            self.ptr = ptr as _;
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            for index in 0..self.len {
                let ptr = self.ptr.wrapping_add(index);
                
                // SAFETY: ...
                unsafe { drop_in_place(ptr) };
            }

            let layout = Layout::array::<T>(self.cap)
                .expect("out of scope!");

            // SAFETY: ...
            unsafe { dealloc(self.ptr as _, layout) };
        }
    }
}

fn main() {
    let mut vec = MyVec::new();
    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));
    vec.push(String::from("4"));
    vec.push(String::from("5"));
    dbg!(vec.pop());
    dbg!(vec.pop());
    dbg!(vec.pop());
}
