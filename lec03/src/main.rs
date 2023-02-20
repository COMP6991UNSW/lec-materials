use std::collections::{HashMap, VecDeque};


fn calculate_mode(vec: Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();

    for elem in vec {
        if map.contains_key(&elem) {
            map.insert(elem, map.get(&elem).unwrap() + 1);
        } else {
            map.insert(elem, 1);
        }
    }

    let (most_frequent_elem, _count) = map.into_iter()
        .max_by_key(|(elem, count)| *count)?;
    
    Some(most_frequent_elem)
}

fn print_string_stuff() {
    let my_string = String::from("Hello, World 😊😊😊!");
    println!("{my_string}");

    for (index, each_character) in my_string.chars().enumerate() {
        println!("{index}: {each_character}");
    }
}

fn main() {
    let a = vec![1, 2, 3, 6, 7, 8, 1, 5, 9, 1, 1, 1];
    let b = vec![1, 2, 3, 5, 7, 8, 1, 5, 2];
    //                    [T, T, T, F, T, T, T, T, F]

    dbg!(longest_equal_run_functional(a, b));
}

fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    let (_current_run, longest_run) = x.into_iter()
        .zip(y.into_iter())     // Iterator<Item = (i32, i32)>
        .map(|(x_value, y_value)| x_value == y_value) // Iterator<Item = bool>
        .fold((0, 0), |(current_run, longest_run), were_the_same| {
            if were_the_same {
                let new_current_run = current_run + 1;
                (new_current_run, usize::max(longest_run, new_current_run))
            } else {
                (0, longest_run)
            }
        });

    longest_run
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut current_run = 0;
    let mut longest_run = 0;

    for (x_value, y_value) in x.into_iter().zip(y.into_iter()) {
        
        if x_value == y_value {
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

fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut current_run = 0;
    let mut longest_run = 0;

    for i in 0..usize::min(x.len(), y.len()) {
        if x[i] == y[i] {
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


/// This function will return None if the vec is empty
/// Otherwise will return Some(average)
fn find_the_mean(some_vec: Vec<i32>) -> Option<f64> {
    if some_vec.is_empty() {
        return None;
    }

    let vec_len = some_vec.len();
    let sum: i32 = some_vec.into_iter().sum();
    let average = (sum as f64) / (vec_len as f64);

    Some(average)
}
