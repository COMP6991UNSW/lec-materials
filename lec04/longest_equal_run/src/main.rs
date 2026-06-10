use itertools::Itertools;

fn main() {
    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ys = vec![1, 2, 1, 4, 5, 6, 2, 3, 9];

    dbg!(longest_equal_run(xs, ys));
    dbg!(longest_equal_run(vec![], vec![]));
}

fn longest_equal_run(xs: Vec<i32>, ys: Vec<i32>) -> usize {
    let mut longest_run = 0;
    let mut current_run = 0;

    for (x, y) in xs.into_iter().zip(ys) {
        if x == y {
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

fn longest_equal_run_functional(xs: Vec<i32>, ys: Vec<i32>) -> usize {
    // (1, 1), (2, 2), (3, 1), (4, 4), ...
    xs.into_iter().zip(ys)
        // T, T, F, T, T, T, F, F, T
        .map(|(x, y)| x == y)
        // (2, T), (1, F), (3, T), (2, F), (1, T)
        .dedup_with_count()
        // (2, T), (3, T), (1, T)
        .filter(|(count, equal)| *equal)
        // 2, 3, 1
        .map(|(count, equal)| count)
        // Some(3)
        .max()
        // 3
        .unwrap_or(0)
}

fn longest_equal_run_imperative(xs: Vec<i32>, ys: Vec<i32>) -> usize {
    let smallest_len;
    if xs.len() < ys.len() {
        smallest_len = xs.len();
    } else {
        smallest_len = ys.len();
    }

    let mut longest_run = 0;
    let mut current_run = 0;

    let mut index = 0;
    while index < smallest_len {
        let x = xs[index];
        let y = ys[index];

        if x == y {
            current_run += 1;
            if current_run > longest_run {
                longest_run = current_run;
            }
        } else {
            current_run = 0;
        }

        index += 1;
    }

    return longest_run;
}




/*
 *    let my_opt = Some(42);
    if my_opt.is_some() {
        let inner_value = my_opt.unwrap();
        println!("Inner value is {inner_value}");
    }

    if let Some(inner_value) = my_opt {
        println!("Inner value is {inner_value}");
    }

    let string = String::from("Hello");
    let mut iter = string.chars();
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    let mut iter = string.chars();
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());

    dbg!(string.chars().all(|c| c.is_lowercase()));

    let new_string: Vec<_> = string.chars()
        .filter(|c| c.is_lowercase())
        .collect();
    dbg!(new_string);

    return;


*/
