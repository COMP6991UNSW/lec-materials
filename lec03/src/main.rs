use std::collections::HashMap;

fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y = vec![1, 0, 3, 4, 5, 6, 20, 8, 9, 10];
    // let average = mean_functional(my_vec)
    //     .expect("Vector is not empty, see line above");

    let l = longest_equal_run_mixed(x, y);
    println!("Longest is {l}");

    let z = vec![1, 1, 1, 2, 1, 5, 5, 5, 8, 5, 7, 5, 6, 5];
    println!("mode is {:?}", mode(z));
}

/// If several elements are equally maximum,
/// the last element is returned.
/// If the Vec is empty, None is returned.
fn mode(vec: Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();

    for elem in vec {
        map.entry(elem)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let (num, _) = map.into_iter()
        .max_by_key(|(_, occurrences)| *occurrences)?;

    Some(num)
}


fn longest_equal_run_imp(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;

    let short_length = usize::min(x.len(), y.len());
    for index in 0..short_length {
        if x[index] == y[index] {
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

fn longest_equal_run_fun(x: Vec<i32>, y: Vec<i32>) -> usize {
    let (best, _) = x.into_iter()
        .zip(y.into_iter())
        .map(|(x_elem, y_elem)| x_elem == y_elem)
        .fold((0, 0), |(best, cur), elem|
            if elem {
                let cur = cur + 1;
                (best.max(cur), cur)
            } else {
                (best, 0)
            }
        );

    best
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;

    for equal in x.into_iter()
        .zip(y.into_iter())
        .map(|(x_elem, y_elem)| x_elem == y_elem) {
        if equal {
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


/// This function returns `None` iff `x` is empty
fn mean_imperative(x: Vec<i32>) -> Option<f64> {
    if x.is_empty() {
        None
    } else {
        let length = x.len();
        let mut overall_sum: i32 = 0;
        
        for elem in x {
            overall_sum += elem;
        }

        Some((overall_sum as f64) / length as f64)
    }
}

fn mean_functional(x: Vec<i32>) -> Option<f64> {
    if x.is_empty() {
        None
    } else {
        let length = x.len();
        let overall_sum: i32 = x.into_iter()
            .sum();

        Some((overall_sum as f64) / length as f64)
    }
}
















#[allow(unused)]
fn double_numbers(vec: Vec<i32>) -> Vec<i32> {
    let mut doubled_vec = Vec::with_capacity(vec.len());

    for elem in vec {
        doubled_vec.push(elem * 2);
    }

    doubled_vec
}
