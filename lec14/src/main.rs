use rayon::iter::{ParallelIterator, IntoParallelIterator};

fn main() {
    let answer: u64 = (0_u64..10_000_000_000)
        .into_par_iter()
        .sum();

    println!("{answer}");
}
