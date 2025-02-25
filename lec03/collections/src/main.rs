#![allow(unused)]

fn main() {
    let xs1 = vec![2, 5, 9, 11, 15, 16];
    let xs2 = vec![-1, 0, 1];

    let mean1 = find_the_mean(xs1.clone());
    let mean2 = find_the_mean(xs2);
    let mean3 = find_the_mean(xs1).unwrap_or(0.0);
    println!("The mean is {mean1:?}");
    println!("The mean is {mean2:?}");
    println!("The mean is {mean3:?}");
}

/// This function will not return NaN
///
/// This function will return None if the provided `vec` is empty.
fn find_the_mean(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    let mut sum = 0;

    for elem in vec.clone() {
        sum += elem;
    }

    Some(sum as f64 / vec.len() as f64)
}

//  sum as f64 / usize::max(vec.len(), 1) as f64
//    if average.is_nan() {
//        return None;
//    }
//
//    Some(average)

fn find_the_median(vec: Vec<i32>) -> f64 {
    // vec.len() => length
    // vec[index]    --  what could go wrong? what type does it return?
    // vec.get(...)  --  what type does it return?

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
