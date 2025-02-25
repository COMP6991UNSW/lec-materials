#![allow(unused)]

use itertools::Itertools;

fn main() {
    // Longest equal run: 3
    let input_1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let input_2 = vec![1, 2, 1, 4, 5, 6, 4, 8];
    
    dbg!(longest_equal_run_mixed(input_1, input_2));
}

fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let len = usize::min(x.len(), y.len());

    let mut current_run = 0;
    let mut longest_run = 0;

    let mut i = 0;
    while i < len {
        if x[i] == y[i] {
            current_run += 1;

            if current_run > longest_run {
                longest_run = current_run;
            }
        } else {
            current_run = 0;
        }
        
        i += 1;
    }

    longest_run
}

// fn is_equal((x, y): (i32, i32)) -> bool {
//     x == y
// }

// [1, 2, 3, 4, 5, 6, 7, 8];
// [1, 2, 1, 4, 5, 6, 4, 8];
fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    x.into_iter().zip(y.into_iter())      // [(1, 1), (2, 2), (3, 1), (4, 4), (5, 5), (6, 6), (7, 4), (8, 8)]
        .map(|(x, y)| x == y)             // [T, T, F, T, T, T, F, T]
        .dedup_with_count()               // [(2, T), (1, F), (3, T), (1, F), (1, T)]
        .filter(|(length, equal)| *equal) // [(2, T), (3, T), (1, T)]
        .map(|(length, equal)| length)    // [2, 3, 1]
        .max()
        .unwrap_or(0)
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut current_run = 0;
    let mut longest_run = 0;

    for (x, y) in x.into_iter().zip(y.into_iter()) {
        if x == y {
            current_run += 1;

            if current_run > longest_run {
                longest_run = current_run;
            }
        } else {
            current_run = 0;
        }
    }

    longest_run
}
