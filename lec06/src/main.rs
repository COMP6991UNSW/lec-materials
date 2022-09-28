#![allow(unused)]

use std::rc::Rc;

mod slice;
// mod dangling;
mod longest;
mod struct_lifetime;
mod elision;

fn foo(string: Rc<String>) {
    
}

fn bar(string: Rc<String>) {

}

fn baz(string: Rc<String>) {

}

fn main() {
    let longer1;
    let longer2;
    let longer3;
    
    {
        let my_string = String::from("Hello World");
        
        let reference_counted: Rc<String> = Rc::new(my_string);

        longer1 = reference_counted.clone();
        longer2 = reference_counted.clone();
        longer3 = reference_counted.clone();
    }

    foo(longer1);
    bar(longer2);
    baz(longer3);
}
