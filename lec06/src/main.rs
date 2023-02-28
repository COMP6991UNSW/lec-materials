#![allow(unused)]

mod slice;
mod dangling;
mod longest;
mod struct_lifetime;
mod elision;

// fn sum_numbers(list: &[i32]) -> i32 {

// }

// str / String --> UTF-8
// [char] --> UTF-32

static GLOBAL: i32 = 42;

fn main() {
    let my_variable = [1, 2, 3, 4, 5];
    dbg!(std::mem::size_of_val(&my_variable));

    // &String --> &str
    // &Vec<T> --> &[T]

    let foo: &'static str = "foo";
    let bar: &'static str = foo;

    {
        let borrow: &'static i32 = &GLOBAL;
    }

    // let my_string = String::from("Hello");
    // my_string = String::from("world");
    // let mut my_string = my_string;    
}


fn foo<'a>(s: &'a str, x: &'a String, y: f32) -> &'a str {
    if true {
        s.trim()
    } else {
        x.as_str()
    }
}

