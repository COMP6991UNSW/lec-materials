#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Student {
    name: String,
    zid: u32,
    // wam: Option<f64>,
}

// Zero-cost abstractions
// - C++
// - Rust
// - ...
//
// Zero-cost meaning:
// 1. If you don't use it, you don't "pay" for it
// 2. If you do use it, it should be as performant as hand-written code

fn main() {
    dbg!(longest_equal_run_imperative(
            vec![6, 9, 9, 1, 1, 2, 3],
            vec![1, 9, 9, 1, 3, 2, 3]));

    dbg!(longest_equal_run_imperative(
            vec![],
            vec![]));

    dbg!(longest_equal_run_imperative(
            vec![1, 2, 3],
            vec![1]));

    dbg!(longest_equal_run_imperative(
            vec![1, 2, 3],
            vec![5]));

    dbg!(longest_equal_run_functional(
            vec![6, 9, 9, 1, 1, 2, 3],
            vec![1, 9, 9, 1, 3, 2, 3]));

    dbg!(longest_equal_run_functional(
            vec![],
            vec![]));

    dbg!(longest_equal_run_functional(
            vec![1, 2, 3],
            vec![1]));

    dbg!(longest_equal_run_functional(
            vec![1, 2, 3],
            vec![5]));

    dbg!(longest_equal_run_mixed(
            vec![6, 9, 9, 1, 1, 2, 3],
            vec![1, 9, 9, 1, 3, 2, 3]));

    dbg!(longest_equal_run_mixed(
            vec![],
            vec![]));

    dbg!(longest_equal_run_mixed(
            vec![1, 2, 3],
            vec![1]));

    dbg!(longest_equal_run_mixed(
            vec![1, 2, 3],
            vec![5]));

    if true { return }

    let mut my_hashmap: HashMap<Student, String> = HashMap::new();

    // my_hashmap.insert(
    //     Student { name: todo!(), zid: todo!() },
    //     String::from("hello"));

    dbg!(find_the_mean(vec![1, 2, 3, 4, 5]));
    dbg!(find_the_mean(vec![1, 2, 3, 4]));
    dbg!(find_the_mean(vec![42]));
    dbg!(find_the_mean(vec![]));

    dbg!(find_the_median(vec![1, 2, 2, 3, 4]));
    dbg!(find_the_median(vec![1, 2, 4, 7]));
    dbg!(find_the_median(vec![42]));
    dbg!(find_the_median(vec![]));
}

// [1, 2, 3, 4]
// sum = 10 / length = 4 => 2.5
fn find_the_mean(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    let mut sum = 0;
    let len = vec.len() as f64;

    for num in vec {
        sum += num;
    }

    Some(sum as f64 / len)
}

// [1, 2, 2, 3, 4] => 2
// [1, 2, 4, 7] => 3
// [] => None
fn find_the_median(mut vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    vec.sort();

    if vec.len() % 2 == 0 {
        // even

        let lower_middle = vec[vec.len() / 2 - 1];
        let upper_middle = vec[vec.len() / 2];

        find_the_mean(vec![lower_middle, upper_middle])
    } else {
        // odd
        
        Some(vec[vec.len() / 2] as f64)
    }
}

// x: [6, 9, 9, 1, 1, 2, 3]
// y: [1, 9, 9, 1, 3, 2, 3]
//
// z: [(6, 1), (9, 9), (9, 9), (1, 1), (1, 3), (2, 2), (3, 3)]
// w: [F,      T,      T,      T,      F,      T,      T]
// w: [F, T, T, T, F, T, T]
//
// Output: 3
//
// x: []
// y: []
// Output: 0
//
// x: [1, 2, 3]
// y: [1]
// Output: 1
//
// x: [1, 2, 3]
// y: [5]
// Output: 0
//
//
// Imperative code: Explaining how to do your task
// Functional code: Describing how to do your task
//
fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let shorter_len = usize::min(x.len(), y.len());

    let mut longest_run = 0;
    let mut current_run = 0;

    let mut i = 0;
    while i < shorter_len {
        let a = x[i];
        let b = y[i];

        if a == b {
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

fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    x.into_iter().zip(y.into_iter()) // -> Iterator<Item = (i32, i32)>
        .map(|(a, b)| a == b)        // -> Iterator<Item = bool>
        .fold((0, 0), |(current, longest), eq| {
            if eq {
                let current = current + 1;
                let longest = usize::max(current, longest);

                (current, longest)
            } else {
                (0, longest)
            }
        }).1
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut current_run = 0;
    let mut longest_run = 0;

    for (a, b) in x.into_iter().zip(y.into_iter()) {
        if a == b {
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

fn word_count(string: &str) -> usize {
    string.split_whitespace().count()
}

fn find_the_mode(vec: Vec<i32>) -> i32 {
    todo!()
}
