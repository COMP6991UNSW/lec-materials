#![allow(unused)]

use std::{thread, time::{Duration, Instant}};

fn main() {
    let mut factor = 2.5;
    let closure = |x: i32| {
        let factor = &mut factor;

        let res = x as f64 * *factor;
        *factor += 1.0;
        res
    };


    let result = [1, 2, 3].into_iter()
        .my_map(closure)
        .collect::<Vec<_>>();

    println!("{result:?}");
}


struct MyMap<I, F> {
    iter: I,
    f: F,
}


trait MyMapExt: Sized {
    fn my_map<F>(self, f: F) -> MyMap<Self, F>;
}

impl<I, Item> MyMapExt for I
where
    I: Iterator<Item = Item>,
{
    fn my_map<F>(self, f: F) -> MyMap<Self, F> {
        MyMap { iter: self, f }
    }
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


