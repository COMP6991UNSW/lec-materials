#![allow(unused)]

use std::{thread::sleep, time::{Duration, Instant}};

fn add1(x: i32) -> i32 {
    x + 1
}

trait MyMapExt<I> {
    fn my_map<F>(self, f: F) -> MyMap<I, F>;
}

impl<I> MyMapExt<I> for I {
    fn my_map<F>(self, f: F) -> MyMap<I, F> {
        MyMap { iter: self, f }
    }
}

struct MyMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, In, Out> Iterator for MyMap<I, F>
where
    I: Iterator<Item = In>,
    F: FnMut(In) -> Out,
{
    type Item = Out;

    fn next(&mut self) -> Option<Out> {
        // Some((self.f)(self.iter.next()?))

        let item: In = self.iter.next()?;
        let mapped: Out = (self.f)(item);

        Some(mapped)
    }
}

fn my_map<I, F>(iter: I, f: F) -> MyMap<I, F> {
    MyMap {
        iter,
        f,
    }
}

fn time_function<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let before = Instant::now();

    f();

    let duration = before.elapsed();
    duration
}

fn slow() {
    println!("Thinking...");
    sleep(Duration::from_secs(1));
    println!("Done!");
}

fn main() {
    /*
    let mut s = String::from("hello");
    let mut closure = || {
        drop(s);
    };

    closure();
    closure();
    */

    let mut s = String::from("hello");
    let duration = time_function(|| {
        println!("Thinking...");
        sleep(Duration::from_secs(1));
        println!("Done!");

        drop(s);
    });
    println!("{duration:?}");

    let mut add = 0;

    let v1 = [1, 2, 3].into_iter()
        .map(|x| {
            add += 1;

            x + add
        })
        .collect::<Vec<_>>();

    println!("{v1:?}");

    let mut add = 0;

    let v2 = [1, 2, 3].into_iter()
        .my_map(|x| {
            add += 1;
            x + add
        })
        .collect::<Vec<_>>();

    println!("{v2:?}");
}

// struct ClosureSrcMain3718 {
//     add: i32,
// }

// impl Fn for ClosureSrcMain3718 {
//     fn call(x: i32) -> i32 {
//         x + self.add
//     }
// }




































/*
fn add1(x: i32) -> i32 {
    x + 1
}

fn call_twice(f: fn()) {
    f();
    f();
}

fn apply_twice<T>(f: fn(T) -> T, t: T) -> T {
    f(f(t))
}

fn main() {
    let f: fn(i32) -> i32 = add1;

    let x = f(42);
    println!("{x}");

    call_twice(|| {
        println!("{}", x);
    });

    let x = apply_twice(add1, 42);
    println!("{x}");
}
*/
