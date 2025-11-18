use std::ptr::{self, null_mut};

struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn push(&mut self, data: T) {
        let node = &mut Node {
            data,
            next: null_mut(),
        } as *mut Node<T>;

        if self.head.is_null() {
            self.head = node;
            self.tail = node;
        } else {
            let curr_tail = unsafe { &mut *self.tail };
            curr_tail.next = node;

            self.tail = node;
        }
    }

    fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                None
            } else if self.head == self.tail {
                let data = ptr::read(self.head).data;

                self.head = null_mut();
                self.tail = null_mut();

                Some(data)
            } else {
                let mut curr = self.head;
                
                while ptr::read(curr).next != self.tail {
                    curr = ptr::read(curr).next;
                }

                let data = ptr::read(self.tail).data;
                ptr::read(curr).next = null_mut();

                Some(data)
            }
        }
    }
}
