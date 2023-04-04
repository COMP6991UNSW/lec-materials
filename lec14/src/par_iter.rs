use std::time::Instant;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    let before = Instant::now();

    let sum: f64 = (1..1000000000000u64).into_par_iter()
        .map(|n| n as f64)
        .sum();

    let duration = Instant::now().duration_since(before);

    println!("Sum: {sum}, took {duration:?}");
}
