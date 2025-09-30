#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

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

fn main() {
    let mut x = String::from("hello");
    
    let shared_borrow_1 = &x;
    let shared_borrow_2 = &x;
    let shared_borrow_3 = shared_borrow_1;
    let shared_borrow_4 = shared_borrow_2;
    let shared_borrow_5 = shared_borrow_4;

    println!("{shared_borrow_1} {shared_borrow_2} {shared_borrow_3} {shared_borrow_4} {shared_borrow_5}");

    x.push_str("!!!");


    let exclusive_borrow_1 = &mut x;
    // let exclusive_borrow_2 = &mut x; // DOESN'T WORK!!!

    println!("{exclusive_borrow_1}");

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
