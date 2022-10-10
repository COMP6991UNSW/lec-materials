use std::collections::LinkedList;

trait Default {
    fn default() -> Self;
}

pub fn main() {
    dbg!(smallest(vec![100,  42, 1000, 42, 7, 600]));
    dbg!(smallest(vec![3.14, 99.9]));
    dbg!(smallest(vec!['a', 'z']));

    let mut l = LinkedList::new();
    l.push_back(42);
    l.push_back(100);
    l.push_back(5000);
    l.push_back(7);
    dbg!(smallest(&l));

    let dwang = Student {
        zid: 1234567,
        name: String::from("dwang"),
        wam: Some(100.0),
    };

    let shupeng = Student {
        zid: 1234566,
        name: String::from("shupeng"),
        wam: Some(100.0),
    };

    dbg!(smallest(vec![dwang, shupeng]));
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// impl<T> Option<T> {
//     fn is_none(&self) -> bool {
//         match self {
//             Self::Some(_) => false,
//             Self::None => true,
//         }
//     }
// }

// impl<T> Option<T>
// where
//     T: Default
// {
//     fn unwrap_or_default(self) -> T {
//         match self {
//             Self::Some(t) => t,
//             Self::None => T::default(),
//         }
//     }
// }



fn smallest<T>(xs: impl IntoIterator<Item = T>) -> Option<T>
where
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


fn foo() {
    smallest_two::<i32>(42, 100);
    smallest_two::<f64>(42.2, 100.1);
    smallest_two::<char>('x', 'z');
}


// Monomorphization

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
