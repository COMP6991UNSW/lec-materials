#![allow(unused)]

fn main() {
    println!(
        "The mean of [1, 2, 3, 4] is {:?}",
        find_the_mean(vec![1, 2, 3, 4])
    );

    println!(
        "The mean of [] is {:?}",
        find_the_mean(vec![])
    );
}

fn find_the_mean(vec: Vec<i32>) -> Option<f64> {
    let mut sum: i32 = 0;
    let len = vec.len();

    if len == 0 {
        return None;
    }

    // for i in 0..vec.len() {
    //     let number = vec[i];
    // }

    for number in vec/*.iter().copied()*/ {
        sum += number
    }

    Some(sum as f64 / len as f64)
}

fn find_the_median(vec: Vec<i32>) -> i32 {
    todo!()
}

fn longest_equal_run(x: Vec<i32>, y: Vec<i32>) -> usize {
    todo!()
}

fn word_count(string: &str) -> usize {
    todo!()
}

fn find_the_mode(vec: Vec<i32>) -> i32 {
    todo!()
}
