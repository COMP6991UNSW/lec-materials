#![allow(unused)]

use std::cmp::max;

use itertools::Itertools;

fn main() {
    // Longest equal run: 3
    let input_1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let input_2 = vec![1, 2, 1, 4, 5, 6, 4, 8];
    //                [T, T, F, T, T, T, F, T]
    //                [(2, T), (1, F), (3, T), (1, F), (1, T)]
    //                [(2, T), (3, T), (1, T)]
    //                [2, 3, 1]
    //                3

    println!("{}", longest_equal_run_mixed(input_1, input_2));
}

fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut i = 0;
    let length;
    if x.len() < y.len() {
        length = x.len();
    } else {
        length = y.len();
    }

    let mut current_run = 0;
    let mut longest_run = 0;

    while i < length {
        let x_elem = x[i];
        let y_elem = y[i];

        if x_elem == y_elem {
            current_run += 1;
            if current_run > longest_run {
                longest_run = current_run;
            }
        } else {
            current_run = 0;
        }

        i += 1;
    }

    return longest_run;
}



// Zero cost abstraction
//
// Internal iteration vs *external iteration*
fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    x.into_iter().zip(y)
        .map(|(x_elem, y_elem)| x_elem == y_elem)
        .dedup_with_count()
        .filter(|(count, same)| *same)
        .map(|(count, same)| count)
        .max()
        .unwrap_or(0)
}



fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut current_run = 0;
    let mut longest_run = 0;

    for (x_elem, y_elem) in x.into_iter().zip(y) {
        if x_elem == y_elem {
            current_run += 1;
            longest_run = max(longest_run, current_run);
        } else {
            current_run = 0;
        }
    }

    longest_run
}
