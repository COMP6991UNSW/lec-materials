use exam_q5_lib::parallel_reduce;

fn main() {
    let items = vec![1, 2, 3, 4, 5];

    let reduced = parallel_reduce(items, 0, |x, y| x + y);
    println!("{reduced}");
}
