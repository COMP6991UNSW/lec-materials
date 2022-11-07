use std::{sync::RwLock, ops::Deref};

struct Foo {
    x: RwLock<i32>,
}

impl Foo {
    fn foo(&self) {
        let my_x = self.x.read().unwrap();
        self.foo_inner(my_x.deref());
    }
    
    fn bar(&self) {
        let my_x = self.x.read().unwrap();
        self.bar_inner(my_x.deref());
    }

    fn foo_inner(&self, my_x: &i32) {
        // a
        // b
        
        self.bar_inner(my_x);

        // f
        // g
    }

    fn bar_inner(&self, my_x: &i32) {
        // c
        // d
        // e
    }
}

pub fn main() {
    
}
