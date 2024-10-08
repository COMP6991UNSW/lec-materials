#![allow(unused)]

use std::collections::LinkedList;

fn main() {
    dbg!(smallest(vec![5, 10]).unwrap());
    dbg!(smallest(vec![12.34, 3.14]).unwrap());
    dbg!(smallest(&vec!['z', 'a']).unwrap());
    dbg!(smallest_char('z', 'a'));
    dbg!(smallest(['z', 'a']).unwrap());
    dbg!(smallest(&['z', 'a']).unwrap());
    dbg!(smallest(LinkedList::from([1, 2, 3])).unwrap());
    dbg!(smallest('f'..='z').unwrap());
    
    dbg!('z'.smallest('a'));
    dbg!('a'.smallest('z'));
}

trait Smallest {
    fn smallest(self, other: Self) -> Self;
}

impl<T> Smallest for T
where 
    T: PartialOrd,
{
    fn smallest(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}


// bounded parametric polymorphism
/// If multiple elements are equal to smallest,
/// this fn will return the first one!
fn smallest<I, T>(xs: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: PartialOrd,
{
    let mut iter = xs.into_iter();
    let mut smallest = iter.next()?;

    for item in iter {
        if item < smallest {
            smallest = item;
        }
    }

    Some(smallest)
}

/*
fn smallest<T>(x: T, y: T) -> T
where 
    T: PartialOrd,
{
    if x < y {
        x
    } else {
        y
    }
}
*/

fn smallest_i32(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_f64(x: f64, y: f64) -> f64 {
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
