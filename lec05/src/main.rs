#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;
mod slice;

fn print_name(name: String) {
    println!("{name}");
}

#[derive(Clone, Copy)]
struct Foo {
    x: i32,
    y: bool,
    z: Option<char>,

}

struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

fn main() {
    let my_string = String::from("Hello");
    println!("{my_string}");


    // Demo temporality of shared xor mutable

    let mut my_string = String::from("Hello");
    my_string.push('!');
    // MUTATION FINISHED
    let my_borrow = &my_string;
    println!("{my_borrow}");
    // SHARING FINISHED
    my_string.push('!');

    // SHARING... FINISHED??
    // println!("{my_borrow}");


    // let mut my_dangerous_option = Some(String::from("Danger"));
    // let my_borrow_of_string = match my_dangerous_option.as_ref() {
    //     Some(string) => string,
    //     None => unreachable!(),
    // };
    // my_dangerous_option = None;
    // println!("{my_borrow_of_string}");

    
    // let my_name = String::from("Zac");
    // print_name(my_name);
    // println!("{my_name}");

    // let harry_name = String::from("Harry");

    // let harry = Student {
    //     name: harry_name,
    //     zid: 1234567,
    //     wam: Some(95.0),
    // };

    // println!("{harry_name}");

    pain::main();
}
