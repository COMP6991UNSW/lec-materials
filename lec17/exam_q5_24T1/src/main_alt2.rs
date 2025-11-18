use std::cell::Cell;

use exam_q5_lib::parallel_reduce;

fn main() {
    let items = [
        Cell::new(1),
        Cell::new(2),
        Cell::new(3),
        Cell::new(4),
        Cell::new(5),
    ];

    let reduced = parallel_reduce(items, Cell::new(1), |x, y| Cell::new(x.get() * y.get()));
    println!("{}", reduced.get());
}
