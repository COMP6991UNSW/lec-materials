#![allow(unused)]

fn main() {
    let xs = vec![1, 2, 3, 4, 5, 6];
    let mean = find_the_mean(xs);
    println!("The mean is {mean}");
}

fn find_the_mean(vec: Vec<i32>) -> f64 {
    let mut sum = 0;

    for elem in vec.clone() {
        sum += elem;
    }

    return sum as f64 / vec.len() as f64;
}

fn find_the_median(vec: Vec<i32>) -> f64 {
    todo!()
}

// fn find_the_mean(vec: Vec<i32>) -> f64 {
//     let mut sum = 0;
//     let len = vec.len();
// 
//     for elem in vec {
//         sum += elem;
//     }
// 
//     return sum as f64 / len as f64;
// }
