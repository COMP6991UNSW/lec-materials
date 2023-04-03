#![allow(unused)]

use std::time::{Duration, Instant};

fn add(x: i32, y: i32) -> i32 {
    x + y
}

trait MyMapExt<I>
where
    I: Iterator,
{
    fn my_map<F, NewItem>(self, f: F) -> MyMap<I, F>
    where
        F: FnMut(<I as Iterator>::Item) -> NewItem;
}

impl<I> MyMapExt<I> for I
where
    I: Iterator,
{
    fn my_map<F, NewItem>(self, map_fn: F) -> MyMap<I, F>
    where
        F: FnMut(<I as Iterator>::Item) -> NewItem
    {
        MyMap { iter: self, map_fn }
    }
}



struct MyMap<I, F> {
    iter: I,
    map_fn: F,
}

impl<I, F, CurrItem, NewItem> Iterator for MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let new_item = (self.map_fn)(item);
        Some(new_item)
    }
}

fn takes_ownership(vec: Vec<i32>) {
    drop(vec);
}

fn time_function<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let before = Instant::now();
    let return_value = f();
    let duration = Instant::now().duration_since(before);

    (return_value, duration)
}

fn foo(x: fn(i32) -> i32) {
    x(42);
}

fn main() {
    let f: fn(i32) -> i32 = |x: i32| x * 2;
    foo(f);


    let (return_value, duration) = time_function::<i32>(|| {
        // let owned = vec![1, 2, 3];
        // let my_fn = || {
        //     takes_ownership(owned);
        // };

        // my_fn();


        let xs = [1, 2, 3, 4, 5];

        let mapped = xs.into_iter()
            .map(|x| (x * 2).to_string())
            .collect::<Vec<_>>();

        println!("std: {mapped:?}");



        let mapped = xs.into_iter()
            .my_map(|x| (x * 2).to_string())
            .collect::<Vec<_>>();

        println!("my:  {mapped:?}");



        {

            let people = ["Zac", "Aaron", "Jerry", "Harry", "Larry"];
            let mut number = 1;

            let mapped = people.into_iter()
                .map(|person| {
                    let greeting = format!("{number}. Hello {person}!");
                    number += 1;
                    greeting
                })
                .collect::<Vec<_>>();

            for greeting in mapped {
                println!("std: {greeting}");
            }
        }


        let people = ["Zac", "Aaron", "Jerry", "Harry", "Larry"];
        let mut number = 1;

        let mapped = people.into_iter()
            .my_map(|person| {
                let greeting = format!("{number}. Hello {person}!");
                number += 1;
                greeting
            }).collect::<Vec<_>>();

        for greeting in mapped {
            println!("my:  {greeting}");
        }

        return 42;
    });

    println!("Main took {duration:?} and returned {return_value:?}");
}







// 
//        foo_exclusive
//     <----------------->
//                   foo_exclusive
//               <------------------>
//
//
//
//
//
//
//
//
//
//
//
//
//
//
