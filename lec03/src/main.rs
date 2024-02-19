#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
struct Student {
    name: String,
    zid: u32,
    // wam: Option<f64>,
}

fn main() {
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

fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    todo!()
}

fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    todo!()
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    todo!()
}

fn word_count(string: &str) -> usize {
    string.split_whitespace().count()
}

fn find_the_mode(vec: Vec<i32>) -> i32 {
    todo!()
}
