#![allow(unused)]

use std::time::{Duration, Instant};

fn get_input_from_user() -> i32 {
    4
}

fn main() {
    let my_string = String::from("hello");
    let my_closure = || {
        std::thread::sleep(Duration::from_secs(3));
        drop(my_string);
    };

    let mut user_input = get_input_from_user();
    let closure = |x| {
        user_input += 1;
        x * user_input
    };

    let xs = [1, 2, 3].into_iter()
        .map(closure)
        .collect::<Vec<_>>();
    dbg!(xs);

    let xs = my_map(
        [1, 2, 3].into_iter(),
        |x| {
            user_input += 1;
            x * user_input
        },
    ).collect::<Vec<_>>();
    dbg!(xs);

    time_function(my_closure);
}

fn time_function(function: impl FnOnce()) {
    let before = Instant::now();
    function();
    let after = Instant::now();

    let duration = after.duration_since(before);
    println!("Function call took {duration:?}");
}

// 0x12340000: MUL a, x, 2
// 0x12340004: RET a
fn double(x: i32) -> i32 {
    x * 2
}

struct MyMap<I, F> {
    iter: I,
    function: F,
}

fn my_map<I, F>(
    iter: I,
    function: F,
) -> MyMap<I, F> {
    MyMap {
        iter,
        function,
    }
}

impl<I, F, In, Out> Iterator for MyMap<I, F>
where
    I: Iterator<Item = In>,
    F: FnMut(In) -> Out,
{
    type Item = Out;

    fn next(&mut self) -> Option<Out> {
        let item = self.iter.next()?;
        let out = (self.function)(item);
        Some(out)
    }
}
