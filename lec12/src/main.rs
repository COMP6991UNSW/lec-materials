#![allow(unused)]

use std::time::{Duration, Instant};

struct MyMap<I, F> {
    iter: I,
    f: F,
}

fn my_map<I, F>(
    iter: I,
    f: F,
) -> MyMap<I, F>
{
    MyMap {
        iter,
        f,
    }
}

impl<I, F, CurrItem, NewItem> Iterator for MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_item = self.iter.next()?;
        let new_item = (self.f)(curr_item);

        Some(new_item)
    }
}

fn time_function<F>(f: F) -> Duration
where
    F: FnOnce()
{
    let before = Instant::now();
    f();
    let after = Instant::now();

    after.duration_since(before)
}

fn main() {
    let mut factor = 3;

    let iter = [5, 10, 20].into_iter();
    let mapped = my_map(iter, |x| {
        let mapped = x * factor;
        factor += 1;
        mapped
    });
    let mapped = my_map(mapped, |x: i32| x.to_string());
    let v = mapped.collect::<Vec<_>>();
    println!("ours: {v:?}");

    let mut factor = 3;

    let v = [5, 10, 20].into_iter()
        .map(|x| {
            let mapped = x * factor;
            factor += 1;
            mapped
        })
        .map(|x: i32| x.to_string())
        .collect::<Vec<_>>();
    println!("std:  {v:?}, factor = {factor}");








    let my_string = String::from("hello");
    let super_dangerous_closure = || {
        println!("{my_string}");
        std::thread::sleep(Duration::from_secs(1));
        drop(my_string);
    };
    let duration = time_function(super_dangerous_closure);
    println!("Super dangerous took {duration:?}");








    let f: fn(i32, i32) -> i32 = add;
    let value = f(2, 3);
    println!("{value}");

    let f = |a: i32, b: i32| {
        a * b * factor
    };

    let add_fn = |x: i32, y: i32| x + y;
    let value = add_fn(2, 3);
    println!("{value}");

    println!("{v:?}");

    if true {
        println!("{v:?}");
    }

    let another_f = || {
        println!("{v:?}");
    };
    another_f();
    another_f();

    do_n_times(|| {
        println!("hello, world");
    }, 5);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// fn hello_world() {
//     println!("hello, world");
// }

fn do_n_times(f: fn(), n: usize) {
    for _ in 0..n {
        f();
    }
}

trait MyMapExt {
    fn my_map<F>(self, f: F) -> MyMap<Self, F>
        where Self: Sized;
}

impl<I> MyMapExt for I
where
    I: Iterator,
{
    fn my_map<F>(self, f: F) -> MyMap<Self, F> {
        MyMap { iter: self, f }
    }
}
