#![allow(unused)]

use std::{num::ParseIntError, ops::IndexMut};

static MY_ARRAY: [i32; 3] = [1, 2, 3];

fn main() {
    let string_literal = "foo";
    let static_slice_borrow = &MY_ARRAY;
    let static_i32_borrow = Box::leak(Box::new(42));

    let mut hello = String::from("hello");
    foo(&mut hello);
    dbg!(hello);

    println!("Hello, world!");
}

fn foo(s: &mut str) {
    
}

fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}
