#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

struct Student {
    name: String,
    zid: u32, // 5555555
    wam: Option<f64>, // 95.3
}

impl Drop for Student {
    fn drop(&mut self) {
        println!("student is being dropped now!!");
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn modify_string(mut string: String) {
    string.push_str("!!!");
}

fn print_point(point: Point) {
    println!("{} {}", point.x, point.y);
}

struct OwnedI32(i32);

fn takes_ownership(student: Student) {
    // ...
}

fn main() {
    let name = String::from("zac");
    let mut student = Student {
        name,
        zid: 5555555,
        wam: Some(13.5),
    };

    let name_borrow = &mut student.name;
    let zid_borrow  = &mut student.zid;
    // let student_borrow = &mut student;

    println!("student name is {name_borrow} and zid is {zid_borrow}");

    let mut x = String::from("hello");

    let shared_borrow_1 = &x;
    let shared_borrow_2 = &x;
    let shared_borrow_3 = shared_borrow_1;
    let shared_borrow_4 = shared_borrow_2;
    let shared_borrow_5 = shared_borrow_4;

    println!("{shared_borrow_1} {shared_borrow_2} {shared_borrow_3} {shared_borrow_4} {shared_borrow_5}");

    x.push_str("!!!");


    let exclusive_borrow_1 = &mut x;
    let exclusive_borrow_2 = exclusive_borrow_1;
    // let exclusive_borrow_2 = &mut x; // DOESN'T WORK!!!

    println!("{exclusive_borrow_2}");

    // let mut y = x.clone();

    // modify_string(x);
    // println!("{x}");

    // int *x = malloc(sizeof(int));
    // *x = 42;
    //
    // ...
    //
    // free(x);

    let x = Box::new(42);
    let y = x;

    let point1 = Point {
        x: *y,
        y: *y,
    };
    print_point(point1);

    // println!("{x}");
    println!("{y}");

    println!("{} {}", point1.x, point1.y);

    let xs = (1, 3.14, true, 'c');
    let ys = xs;
    println!("{xs:?} {ys:?}");
}
