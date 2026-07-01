#![allow(unused)]

use std::{collections::HashSet, fmt::{Display, format}, ops::Add};

struct ZacsInt(i32);

impl From<i32> for ZacsInt {
    fn from(value: i32) -> ZacsInt {
        ZacsInt(value)
    }
}

// impl Into<i32> for ZacsInt {
//     fn into(self) -> i32 {
//         self.0
//     }
// }

impl Display for ZacsInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZacsInt of {}", self.0)
    }
}

fn takes_a_zac_int(x: impl Into<ZacsInt>) {
    let x = x.into();
    println!("{x}");
}

fn main() {
    takes_a_zac_int(42);

    let x: ZacsInt = 42.into();
    takes_a_zac_int(x);

    // println!("{x}");
    // // let x_string = format!("{x}");
    // let x_string = x.to_string();
    // println!("{x_string}");

    let x = 1 + 2;
    let x = Add::add(1, 2);

    dbg!(smallest_list(vec![5, 0, 3]).unwrap());
    dbg!(smallest_list([2.50, 3.14, 1.25]).expect("exactly 2 inputs given"));
    dbg!(smallest_list(HashSet::from(['z', 'a'])).unwrap_or_default());

    let my_smallest_char: fn(char, char) -> char
        = smallest_char;
    let the_char = my_smallest_char('c', 'f');
    dbg!(the_char); // 'c'
}

fn smallest_list<I, T>(xs: I) -> Option<T>
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

// impl trait, does it work for smallest?
// Maybe we can use From / Into to make the interface nicer?

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

//  if PartialOrd::partial_cmp(&x, &y) == Some(Ordering::Less) {

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

/*
enum Either<T, U> {
    Left(T),
    Right(U),
}

fn smallest<T, U>(x: T, y: U) -> Either<T, U>
where
    T: PartialOrd<U>,
{
    if x < y {
        Either::Left(x)
    } else {
        Either::Right(y)
    }
}
*/
