#![allow(unused)]

fn main() {
    let mut vec = vec![1, 2, 3];

    println!("{:?}", vec);
    let mean = find_the_mean(vec);
    println!("{mean:?}");

    let mut vec = vec![1, 3, 4, 5, 9, 10, 11];
    let median = find_the_median(vec);
    println!("{median:?}");
}

// [1, 2, 3, 4, 5] -> 3
// 1 + 2 + 3 + 4 + 5 == 15 / 5 => 3
// [1, 2, 3, 4] -> 2.5
/// This function will return None iff the input `vec` is empty.
fn find_the_mean(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    let mut total = 0.0;

    for elem in vec.clone() {
        total += elem as f64;
    }

    let mean = total / vec.len() as f64;

    Some(mean)
}


// [1, 2, 3, 4, 5] -> 3
// [1, 3, 4, 5, 9, 10, 11] -> 5
// [1, 2, 3, 4] -> 2.5
// [1] -> 1
// [] -> None
fn find_the_median(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    let len = vec.len();
    if len % 2 == 0 {
        let lower = len / 2 - 1;
        let upper = lower + 1;

        let mean = find_the_mean(vec![vec[lower], vec[upper]])
            .expect("the vec is not empty");

        Some(mean)
    } else {
        Some(vec[len / 2] as f64)
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



fn word_count(string: String) -> usize {
    todo!()
}



fn find_the_mode(vec: Vec<i32>) -> i32 {
    todo!()
}
