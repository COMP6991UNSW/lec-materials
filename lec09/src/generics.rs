use std::{cmp::PartialOrd, collections::LinkedList};

pub fn main() {
    dbg!(smallest_two_i32(100, 42));
    dbg!(smallest_two_f32(3.14, 99.9));
    dbg!(smallest_two_char('a', 'z'));

    dbg!(smallest_two::<i32>(100, 42));
    dbg!(smallest_two::<f32>(3.14, 99.9));
    dbg!(smallest_two::<char>('a', 'z'));

    dbg!(smallest_two(100, 42));
    dbg!(smallest_two(3.14, 99.9));
    dbg!(smallest_two('a', 'z'));

    let v: Vec<i32> = vec![];

    dbg!(smallest_n(vec![5, 3, 2, 1, 4]));
    dbg!(smallest_n(vec![5, 6, 7]));
    dbg!(smallest_n(vec![7]));
    dbg!(smallest_n(vec!["hello", "world", "foo", "bar"]));
    dbg!(smallest_n(v));
    dbg!(smallest_n(LinkedList::from([1, 2, 3])));
    
    // let f1 = File::open("foo1.txt");
    // let f2 = File::open("foo2.txt");
    // smallest_two(f1, f2);
}

// fn smallest_n_or_max<T>(list: Vec<T>) -> T
// where
//     T: PartialOrd,
//     T: Max
// {
//     smallest_n(list).unwrap_or(T::max())
// }

// fn smallest_n<T>(list: &[T]) -> Option<&T> { ... }

fn smallest_n<Iter, Item>(list: Iter) -> Option<Item>
where
    Iter: IntoIterator<Item = Item>,
    // Iter: IntoIterator<Item = T>,
    Item: PartialOrd,
{
    let mut iter = list.into_iter();
    let mut smallest = iter.next()?;

    for item in iter {
        if item < smallest {
            smallest = item;
        }
    }

    Some(smallest)
}

// What restrictions exist on X, Y, Z?
// 1. Z = X (in case x is smaller)
// 2. Z = Y (in case y is smaller)
//   => X = Y = Z = T
// 3. T must be comparable
fn smallest_two<T>(x: T, y: T) -> T
where
    T: PartialOrd,
{
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_i32(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_f32(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_char(x: char, y: char) -> char {
    if x < y {
        x
    } else {
        y
    }
}

// fn print_debug_repr<T>(value: T)
// where
//     T: Debug,
// {
//     println!("{value:?}");
// }
// 
// fn print_debug_repr(value: impl Debug)
// {
//     println!("{value:?}");
// }


// Option: fn(Type) -> Type
//         fn(i32)  -> Option<i32>
//         fn(String) -> Option<String>
//         fn(T)    -> Option<T>

// enum Option<T> {
//     None,
//     Some(T),
// }

// impl<T> Option<T> {
//     fn foo() {
//         println!("Foo!");
//     }
// }

