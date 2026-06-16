#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

// String  x ---------->    ['4', '2', ...., ..., .,.,,.]
//                            
//                           
// String y -----------------> ['4', '2', ...., ..., .,.,,.]

// fn my_drop(string: String) { }


fn main() {
    let x = String::from("42");
    let y = x;
    println!("{y}");

    let foo = Box::new(42);


    let mut x = String::from("123");
    let shared_1 = &x;
    let shared_2 = shared_1;
    let shared_3 = shared_2;
    let shared_4 = shared_3;
    let shared_5 = shared_4;
    println!("{shared_1} {shared_2} {shared_3} {shared_4} {shared_5} ");

    let exclusive = &mut x;
    exclusive.push_str("45");
    let exclusive_2 = exclusive;
    let exclusive_3 = exclusive_2;
    let exclusive_4 = exclusive_3;
    let exclusive_5 = exclusive_4;
    println!("{exclusive_5}");

    // println!("{x}");
    // println!("{y}");


    // let mut x = 42;
    // let eb1 = &mut x;
    // let eb2 = &mut x;
    // println!("{eb1}");
    // println!("{eb2}");
}



















































// fn duplicate_string(string: String) -> (String, String) {
//     // new String: second_string
//     // copies all the characters from first to second
//     // gives them both back
// }


// fn foo(x: &i32) -> &i32 {
//     x
// }
// 
// fn longest(x: &str, y: &str) -> &str {
//     
// }
// 
// struct Foo<'a> {
//     x: &'a i32,
// }
