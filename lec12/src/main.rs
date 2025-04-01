#![allow(unused)]

use std::{thread, time::{Duration, Instant}};

fn main() {
    let mut factor = 2.5;
    let result = [1, 2, 3].into_iter()
        .map(|x| {
            let factor = &mut factor;

            let res = x as f64 * *factor;
            *factor += 1.0;
            res
        })
        .collect::<Vec<_>>();

    println!("{factor} {result:?}");

    time_function(|| {
        thread::sleep(Duration::from_secs(3));

        let mut factor = 2.5;
        let closure = |x: i32| {
            let factor = &mut factor;

            let res = x as f64 * *factor;
            *factor += 1.0;
            res
        };

        let result = my_map(
            [1, 2, 3].into_iter(),
            closure,
        )
        .collect::<Vec<_>>();

        println!("{result:?}");
    });
}

fn time_function<F>(f: F)
where
    F: FnOnce(),
{
    let before = Instant::now();
    f();
    let after = Instant::now();

    let duration = after.duration_since(before);
    println!("Function took {duration:?}");
}

struct MyMap<I, F> {
    iter: I,
    f: F,
}

fn my_map<I, F>(iter: I, f: F) -> MyMap<I, F> {
    MyMap { iter, f }
}

impl<I, F, CurrItem, NewItem> Iterator for MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.iter.next()?;
        let new = (self.f)(curr);

        Some(new)
    }
}











// fn mult(x: i32) -> f64 {
//     x as f64 * 2.5
// }




/*
    let f: fn(i32, i32) -> i32 = add;
    let x: i32 = f(2, 3);
    let y: i32 = f(3, 4);

    println!("{x} {y}");

    let my_closure = || {
        println!("hello!");
    };
    my_closure();

    let closure_add = |x: i32, y: i32| x + y;

    // print_hello();
    call_n(my_closure, 2);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn compose<A, B, C>(f: fn(A) -> B, g: fn(B) -> C, a: A) -> C {
    let b = f(a);
    let c = g(b);

    c
}

fn call_n(f: fn(), n: usize) {
    for _ in 0..n {
        f();
    }
}

fn print_hello() {
    println!("hello!");
}


    let a = 42;
    let b = 3.14;
    let c = 'z';
    let d = String::from("hello");

    let f: fn(i32) -> i32 = |x| {
        println!("{a} {b} {c} {d}");
        x * 2
    };
    let g: fn(i32) -> i32 = |x| {
        println!("{a}");
        x + 5
    }; // <--------.
    // 0x81273981723     the location of the code -|


    let mut f: fn(i32) -> i32 = |x| x * 2;
    f = |x| x + 5;




*/
