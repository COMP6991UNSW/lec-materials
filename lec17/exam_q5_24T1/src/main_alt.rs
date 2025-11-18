use std::collections::LinkedList;

use exam_q5_lib::parallel_reduce;

#[derive(Clone)]
struct MyBool(bool);

fn main() {
    let items = LinkedList::from([MyBool(true), MyBool(false), MyBool(true), MyBool(false), MyBool(true)]);

    let MyBool(reduced) = parallel_reduce(items, MyBool(false), |MyBool(x), MyBool(y)| MyBool(x ^ y));
    println!("{reduced}");
}
