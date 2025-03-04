#![allow(unused)]

// mod dangling;
// mod slice;
mod longest;

struct Foo {
    x: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping Foo with x={}", self.x);
    }
}

fn drop<T>(t: T) {}

fn main() {

    let b = Box::new(42);
    // int *b = malloc(sizeof(int));
    // *b = 42;

    println!("before scope");
    {
        let foo = Foo { x: 42 };

    }
    println!("after scope");
}










/*
fn main() {
    let mut student = Student { name: String::from("zac"), address: String::from("hello st"), ph: String::from("04"), marks: vec![] };
    
    let excl_borrow = &mut student;
    let marks = excl_borrow.marks;

    add_mark(&mut excl_borrow.marks, 100.0);

    let stolen_name = student.name;
    student.name = String::from("caz");

    let stolen_address = &student.address;
    let stolen_ph = &student.ph;
    let stolen_address = &student.address;

    takes_student(student);
}

fn takes_student(student: Student) { todo!() }

struct Student {
    name: String,
    address: String,
    ph: String,
    marks: Vec<f64>,
}

fn add_mark(student: &Student, mark: f64) {
    let marks_vec = &student.marks;
    Vec::push(&mut student.marks, mark);
    student.marks.push(mark);
}
*/
