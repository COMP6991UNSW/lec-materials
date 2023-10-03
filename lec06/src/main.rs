#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

fn main() {
    let student_name = String::from("Delph");

    let mut student = Student {
        zid: 5555555,
        name: student_name,
        wam: None,
    };

    set_wam_to_100(&mut student);
    say_hello_to_student(&student);
}


fn say_hello_to_student(borrow_student: &Student) {
    println!("Hello there {}, your wam is {:?}", borrow_student.name, borrow_student.wam);
}

fn actually_takes_ownership(student: Student) {
    todo!()
}

fn just_takes_a_borrow(student: &Student) {
    todo!()
}

fn set_wam_to_100(student: &mut Student) {
     student.wam = Some(100.0);
}
