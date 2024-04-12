// BAD CODE DO NOT USE!!!

use std::rc::Rc;

struct Foo {
    rc: Rc<i32>,
}

// Safety: This is why this is ok
unsafe impl Send for Foo {}

fn main() {
    let rc = Rc::new(42);
    let rc_clone = Rc::clone(&rc);

    let foo = Foo { rc };

    std::thread::spawn(move || {
        let x = foo;
        let x = x.rc;
    });

    let x = rc_clone;

    println!("Hello, world!");
}
