#![allow(unused)]

use std::{
    fmt::{Debug, Display, Error, Formatter},
    ops::{Add, Mul},
    thread::current,
};

pub fn main() {
    dbg!(smallest(3, 2));
    dbg!(smallest('a', 'b'));
    dbg!(smallest(3.5, 3.7));
    let my_vec: Vec<i32> = vec![];
    dbg!(smallest_from_vec(my_vec));

    // default_testing();

    let vec = vec![1, 2, 3];
    let iter = vec.into_iter();

    // test_fibonacci();
    test_iterator();
}

// fn odd_integers(start: u32, stop: u32) -> impl Iterator<Item = u32> {
//     if start == 0 {
//         (start..stop).filter(|i| i % 2 != 0).map(|x| x * 1)
//     } else {
//         (start..stop).map(|x| x * 2)
//     }
// }
enum MyInputs {
    String(String),
    Num(i32),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// void qsort(void* array, size_t size, int(void*, void*) cmp);

fn qsort<I>(iter: impl IntoIterator<Item = I>) -> Vec<I>
where
    I: PartialOrd,
{
    todo!()
}

fn smallest_from_vec<T>(v: impl IntoIterator<Item = T>) -> Option<T>
where
    T: PartialOrd + Debug,
{
    let mut iter = v.into_iter();
    let mut lowest = iter.next()?;

    for value in iter {
        if value < lowest {
            lowest = value;
        }
    }

    return Some(lowest);
}

fn smallest<T>(x: T, y: T) -> T
where
    T: PartialOrd + std::fmt::Debug,
{
    if x < y {
        x
    } else {
        y
    }
}

// #[derive()]

#[derive(Default, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Coordinate3d {
    x: i32,
    y: i32,
    z: i32,
}

impl From<Coordinate> for Coordinate3d {
    fn from(value: Coordinate) -> Self {
        Coordinate3d {
            x: value.x,
            y: value.y,
            z: 0,
        }
    }
}

/*impl Into<Coordinate3d> for Coordinate {
    fn into(self) -> Coordinate3d {
        Coordinate3d {
            x: self.x,
            y: self.y,
            z: 0,
        }
    }
}*/
/*
impl<T: From<U>, U> Into<T> for U {
    fn into(self) -> T {
        T::from(self)
    }
}*/

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: i32) -> Coordinate {
        Coordinate {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// impl Clone for Coordinate {
//     fn clone(self: &Self) -> Self {
//         Self {
//             x: self.x.clone()
//             y: self.y.clone()
//         }
//     }
// }

fn default_testing() {
    let coord = Coordinate::default();

    dbg!(coord.to_string());
    dbg!(3.to_string_2());

    let x = 1 + 2;
}

trait ToString6991 {
    fn to_string_2(&self) -> String;
}

impl<T: Display> ToString6991 for T {
    fn to_string_2(&self) -> String {
        self.to_string()
    }
}

// trait Add<T> {
//     fn add(self, rhs: T) -> T;
// }

// impl Add for i32 {
//     fn add(self, rhs: i32) -> i32 {
//         self + rhs
//     }
// }

trait AddOne {
    type Output;
    fn adds_one(self) -> Self::Output;
}

impl AddOne for i32 {
    type Output = i32;

    fn adds_one(self) -> Self::Output {
        self + 1
    }
}

impl AddOne for &i32 {
    type Output = i32;

    fn adds_one(self) -> Self::Output {
        (*self) + 1
    }
}

fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

fn is_hello_(s: &str) {
    assert_eq!("hello", s);
}

struct Fibonacci {
    curr: i32,
    next: i32,
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.curr;
        let n = self.next;

        self.curr = n;
        self.next = c + n;

        if n > 10000000 {
            None
        } else {
            Some(c)
        }
    }
}

fn test_fibonacci() {
    let mut fib = Fibonacci { curr: 0, next: 1 };
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());
    dbg!(fib.next());

    dbg!(fib.collect::<Vec<_>>());
}

struct VectorIterator<'a, T> {
    vec: &'a Vec<T>,
    position: usize,
}

impl<'b, T> Iterator for VectorIterator<'b, T> {
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pos = self.position;
        self.position += 1;
        if current_pos >= self.vec.len() {
            None
        } else {
            Some(&self.vec[current_pos])
        }
    }
}

fn test_iterator() {
    let my_vec = vec![1, 2, 3];
    let mut my_vec_iter = VectorIterator {
        vec: &my_vec,
        position: 0,
    };

    for x in my_vec_iter {
        dbg!(x);
    }
}
