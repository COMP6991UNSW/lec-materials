// fn return_ptr() -> &i32 {
//     let x = 42;
//     &x
// }

use std::{cell::Cell, thread};

struct Foo {
    cell: Cell<i32>,
}

// Safety: This is ok because ...
unsafe impl Sync for Foo {}

fn main() {
    let foo = Foo { cell: Cell::new(42), };
    thread::scope(|scope| {
        scope.spawn(|| {
            let foo = &foo;
            foo.cell.set(100);
        });
    });

    let mut v = vec![1, 2, 3];

    let x = {
        println!("I'm in a block!");

        42
    };

    println!("I'm in an unsafe block!");

    // /!\ Safety: 
    // Requirements of `Vec::set_len`:
    // §
    // Safety
    // new_len must be less than or equal to capacity().
    // The elements at old_len..new_len must be initialized.
    //
    // `len` is always <= `capacity`, the new len is less than
    // the old len, therefore <= `capacity`.
    //
    // This is ok, because we know that the vec has a len of 3,
    // and shrinking the len is valid
    let x = unsafe {

        v.set_len(2);

        42
    };
}
