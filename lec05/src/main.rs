#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

fn main() {
    let mut my_string = String::from("hello");
    dbg!(my_string);
}
