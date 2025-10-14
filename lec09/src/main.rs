#![allow(unused)]
use core::panic;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, ops::Add};

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

    let xs = [1, 2, 3].into_iter()
        .map(|x| x * 2)
        .collect::<HashSet<_>>();

    let map: HashMap<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into_iter()
        .collect();

    // dbg!(smallest::<i32>(vec![]));
    dbg!(smallest([3, 5, 8, 2]).unwrap());
    dbg!(smallest(HashSet::from([10, 3, 100, 5])).expect("vec is non-empty"));
    dbg!(smallest(vec!['z', 'a', '!', '_', ',', '@', 'C']).unwrap_or('#'));
    dbg!(smallest(vec![Student { name: String::from("bobby"), zid: 5555555, wam: Some(100.0) },
        Student { name: String::from("tommy"), zid: 6666666, wam: Some(0.0) }]).unwrap_or_default());
}

fn smallest<I, T>(xs: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
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

// fn smallest<T>(x: T, y: T) -> T
// where
//     T: PartialOrd,
// {
//     println!("smallest called with {}", std::any::type_name::<T>());
//     if x < y {
//         x
//     } else {
//         y
//     }
// }

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
