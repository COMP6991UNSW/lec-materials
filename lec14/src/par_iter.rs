use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn main() {
    let sum: f64 = (1..100000000000u64).into_par_iter()
        .map(|n| n as f64)
        .sum();

    println!("{sum:?}");
}
