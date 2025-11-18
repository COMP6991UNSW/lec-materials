use std::sync::Mutex;

use exam_q5_lib::parallel_reduce;

fn main() {
    let mutex = Mutex::new(5);
    let lock = mutex.lock().unwrap();
    let items = [1, 2, 3, 4, 5];

    let reduced = parallel_reduce(items, 0, |x, y| x + y + *lock);
    println!("{reduced}");
}
