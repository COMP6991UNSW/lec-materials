use itertools::Itertools;

fn main() {
    dbg!(longest_equal_run_functional(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 0, 4, 5, 6, 2, 3, 9]));
    //           T, T, F, T, T, T, F, F, T
}

fn longest_equal_run_imperative(xs: Vec<i32>, ys: Vec<i32>) -> usize {
    let smallest_len;
    if xs.len() < ys.len() {
        smallest_len = xs.len();
    } else {
        smallest_len = ys.len();
    }

    let mut current_run = 0;
    let mut longest_run = 0;

    let mut index = 0;
    while index < smallest_len {
        let x = xs[index];
        let y = ys[index];
        if x == y {
            current_run += 1;
            if current_run > longest_run {
                longest_run = current_run;
            }
        } else {
            current_run = 0;
        }

        index += 1;
    }

    return longest_run;
}

fn longest_equal_run_functional(xs: Vec<i32>, ys: Vec<i32>) -> usize {
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    // [1, 2, 0, 4, 5, 6, 2, 3, 9]
    xs.into_iter().zip(ys)    // [(1, 1), (2, 2), (3, 0), (4, 4), ...]
        .map(|(x, y)| x == y) // [T, T, F, T, ...]
        .dedup_with_count()
        .filter(|(length, equal)| *equal)
        .map(|(length, equal)| length)
        .max()
        .unwrap_or(0)
}
