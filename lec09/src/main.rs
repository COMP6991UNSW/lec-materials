#![allow(unused)]
use core::panic;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, iter::{Filter, Map, Peekable, Zip}, ops::Add};

#[derive(Debug, PartialEq, PartialOrd)]
struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

impl Default for Student {
    fn default() -> Self {
        Self { name: Default::default(), zid: Default::default(), wam: Default::default() }
    }
}

fn main() {
    // let s = smallest::<String>;
    // let t = s("a".to_string(), "b".to_string());

    let f = foo();
    println!("{}", std::any::type_name_of_val(&f));
    let i = f.into_iter();
    for x in i {
        println!("{x}");
    }

    let xs = [1, 2, 3].into_iter()
        .map(|x| x * 2)
        .collect::<HashSet<_>>();

    let map: HashMap<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into_iter()
        .collect();

    let xs = transform(vec![1, 2, 3]);

    dbg!(smallest::<i32>(vec![]));
    dbg!(smallest([3, 5, 8, 2]).unwrap());
    dbg!(smallest(HashSet::from([10, 3, 100, 5])).expect("vec is non-empty"));
    dbg!(smallest(vec!['z', 'a', '!', '_', ',', '@', 'C']).unwrap_or('#'));
    dbg!(smallest(vec![Student { name: String::from("bobby"), zid: 5555555, wam: Some(100.0) },
        Student { name: String::from("tommy"), zid: 6666666, wam: Some(0.0) }]).unwrap_or_default());
}

// APIT: Argument-Position Impl Trait
// T: i32, I: Vec<i32>
// T: i32, I: HashSet<i32>
// T: i64, I: Vec<i64>
fn smallest<T>(xs: impl IntoIterator<Item = T>) -> Option<T>
where
    T: PartialOrd,
{
    let mut iter = xs.into_iter();
    let mut least = iter.next()?;

    for elem in iter {
        if elem < least {
            least = elem;
        }
    }

    Some(least)
}

// RPIT: Return Position Impl Trait
fn transform(xs: Vec<i32>) -> impl Iterator<Item = (i32, i32)> {
    let ys = xs.into_iter()
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0)
        .zip(0..)
        .peekable();

    ys
}

fn transform2(xs: Vec<i32>) -> Vec<(i32, i32)> {
    let ys = xs.into_iter()
        .map(|x| x * 2)
        .filter(|x| x % 3 == 0)
        .zip(0..)
        .peekable()
        .collect();

    ys
}

fn foo() -> impl IntoIterator<Item = i32> {
    return HashSet::from([1, 2, 3]);
}

// fn main2() {
//     dbg!(smallest2_i32(3, 5));
//     dbg!(smallest2_f32(3.0, 5.5));
//     dbg!(smallest2_char('a', 'z'));
//     dbg!(smallest2_student(Student { ... }, Student { ... }));
// }

// T: i32
// T: f64
// T: char
// T: Student
fn smallest2<T>(x: T, y: T) -> T
where
    T: PartialOrd,
{
    println!("smallest called with {}", std::any::type_name::<T>());
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_i32(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_f32(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_char(x: char, y: char) -> char {
    if x < y {
        x
    } else {
        y
    }
}
