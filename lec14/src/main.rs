use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

fn main() {
    let total: f64 = (0..100_000_000_000u64)
        .into_par_iter()
        .map(|n| n as f64)
        .sum();

    println!("{total}");
}
