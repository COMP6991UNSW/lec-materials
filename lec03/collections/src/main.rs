#![allow(unused)]

fn main() {
    // let mut my_vec = vec![1, 2, 3];
    // 
    // let mut iter = my_vec.iter();
    // 
    // my_vec.push(42);

    // let elem = iter.next();



    println!(
        "The mean of [1, 2, 3, 4] is {:?}",
        find_the_mean(vec![1, 2, 3, 4])
    );

    println!(
        "The mean of [] is {:?}",
        find_the_mean(vec![])
    );

    println!(
        "The median of [2, 5, 1, 7] is {:?}",
        find_the_median(vec![2, 5, 1, 7])
    );

    println!(
        "The median of [] is {:?}",
        find_the_median(vec![])
    );

    let x = vec![6, 9, 9, 1, 5, 6, 7, 8, 1, 2, 3];
    let y = vec![1, 2, 3, 1, 5, 6, 1, 8, 1, 2, 3, 1, 2, 3];
    println!(
        "longest_equal_run\n{x:?}\n{y:?}\nAnswer: {:?}",
        longest_equal_run_mixed(x.clone(), y.clone())
    );

    println!("Word count: {}", word_count("hello this is cs6991 and we're learning about iteration!"));
}

fn find_the_mean(vec: Vec<i32>) -> Option<f64> {
    let mut sum: i32 = 0;
    let len = vec.len();

    if vec.is_empty() {
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

fn find_the_median(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }

    // len | middle index
    // 1   | 0
    // 2   | 0-1
    // 3   | 1
    // 5   | 2
    // 6   | 2-3
    // 7   | 3

    let len = vec.len();
    if len % 2 == 0 {
        // even case

        let lower_middle_index = len / 2 - 1;
        let upper_middle_index = len / 2;

        find_the_mean(vec![vec[lower_middle_index], vec[upper_middle_index]])
    } else {
        // odd case

        let middle_index = len / 2;
        Some(vec[middle_index] as f64)
    }
}

// x: [6, 9, 9, 1, 5, 6, 7, 8, 1, 2, 3]
// y: [1, 2, 3, 1, 5, 6, 1, 8, 1, 2, 3, 1, 2, 3]
// r: [F, F, F, T, T, T, F, T, T, T, T]
//
// output -> 4
fn longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;

    let mut index = 0;

    let shortest_len = usize::min(x.len(), y.len());

    for index in 0..shortest_len {
        if x[index] == y[index] {
            current_run += 1;
        } else {
            if current_run > longest_run {
                longest_run = current_run;
            }
            current_run = 0;
        }
    }
    if current_run > longest_run {
        longest_run = current_run;
    }

    longest_run
}

// x: [6, 9, 9, 1, 5, 6, 7, 8, 1, 2, 3]
// y: [1, 2, 3, 1, 5, 6, 1, 8, 1, 2, 3, 1, 2, 3]
// r: [F, F, F, T, T, T, F, T, T, T, T]
//
// output -> 4
fn longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    let (_, longest_run) = x.into_iter().zip(y.into_iter())
        .map(|(x_item, y_item)| x_item == y_item)
        .fold((0, 0), |(current_run, longest_run), elem| {
            if elem {
                let new_current_run = dbg!(current_run) + 1;
                let new_longest_run = usize::max(longest_run, new_current_run);

                (new_current_run, new_longest_run)
            } else {
                (0, longest_run)
            }
        });

    longest_run
}

fn longest_equal_run_mixed(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;

    for same in x.into_iter().zip(y.into_iter())
        .map(|(x_item, y_item)| x_item == y_item) {
        if same {
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
