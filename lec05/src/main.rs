#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;
mod slice;


fn main() {
    pain::main();





















    let string = String::from("dwang");

    let student = Student {
        zid: 5555555,
        name: string,
        wam: Some(100.0),
    };

    say_hello_to_student(student);
    // say_hello_to_student(student);

    let x = 42; // i32: Copy
    // i32, u32, i8, i128, f32, f64, bool, char, (), (bool, i32), (i32, i32, i32, ..., i32)
    let y = x.clone();

    println!("x = {x}, y = {y}");
}


struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

enum Foo {
    A(String),
    B(Student),
    C(MoveI32),
}

fn say_hello_to_student(student: Student) {
    println!("Hello there {}", student.name);
}

#[derive(Copy, Clone)]
struct MoveI32(i32);
