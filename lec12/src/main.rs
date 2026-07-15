#![allow(unused)]

fn main() {
    let mut f: fn(i32, i32) -> i32 = add;
    f = sub;
    let answer = f(2, 3);
    dbg!(answer);

    [1, 2, 3].into_iter()
        .map(|x| x * 2)
        .collect::<Vec<_>>();

    call_fn_n_times(print_hello, 10);
    dbg!(reduce([1, 2, 3, 4, 5], add));
    dbg!(reduce([1, 2, 3, 4, 5], |x, y| x + y));
    dbg!(reduce([1, 2, 3, 4, 5], sub));
}

fn reduce<T>(
    items: impl IntoIterator<Item = T>,
    function: fn(T, T) -> T,
) -> Option<T>
{
    let mut iter = items.into_iter();

    let mut lhs = iter.next()?;
    for rhs in iter {
        // (lhs, rhs)
        lhs = function(lhs, rhs);
    }

    Some(lhs)
}

fn call_fn_n_times(function: fn(), n: usize) {
    for _ in 0..n {
        function();
    }
}

fn print_hello() {
    println!("hello there!");
}

// [1, 2, 3, 4, 5]
// (1, 2) [3, 4, 5]
// (3, 3) [4, 5]
// (6, 4) [5]
// (10, 5) []
// --> 15
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}
