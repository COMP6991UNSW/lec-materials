use std::{fs::File, cmp::Ordering, collections::HashSet, hash::Hash};

pub fn main() {
    dbg!(smallest_two_i32(100,  42));
    dbg!(smallest_two_f32(3.14, 99.9));
    dbg!(smallest_two_char('a', 'z'));

    dbg!(smallest_two(100,  42));
    dbg!(smallest_two(3.14, 99.9));
    dbg!(smallest_two('a', 'z'));

    let student1 = Student {
        zid: 5555555,
        name: String::from("Test student"),
        wam: None,
    };

    let student2 = Student {
        zid: 4444444,
        name: String::from("I love 4 student"),
        wam: None,
    };

    dbg!(smallest_two(student1, student2));

    dbg!(smallest(&vec![5, 3, 1, 2, 4]));
    dbg!(smallest(&vec![2, 2, 2, 2]));
    dbg!(smallest(&vec![4]));
    dbg!(smallest(&Vec::<i32>::new()));
    dbg!(smallest(&[5, 3, 1, 2, 4]));
    dbg!(smallest(&[2, 2, 2, 2]));
    dbg!(smallest(&[4]));
    let x: [i32; 0] = [];
    dbg!(smallest(&x));
    
    let mut set = HashSet::new();
    set.insert(5);
    set.insert(4);
    set.insert(1);
    set.insert(2);
    set.insert(3);

    dbg!(smallest(set));

    print_all(&[1, 2, 3, 4, 5]);
    print_all(&["foo", "bar", "baz"]);
    // print_all(&[1, "bar", 2, "baz"]);

    // dbg!(smallest([1, 5, 3.2, 'z', student1]));
    // dbg!(smallest_two(File::open("foo"), File::open("bar")));
}

#[derive(Debug)]
struct Student {
    zid: u32,
    name: String,
    wam: Option<f32>,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.zid == other.zid
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.zid.partial_cmp(&other.zid)
    }
}

/*
struct MyVec<T> {
    // ...
}

impl<T> MyVec<T> {
    pub fn new(first_t: T) -> Self {
        Self {
            // ...
        }
    }

    pub fn remove_elem(/* ... */) {
        if self.len() == 1 {
            return;
        }

        // remove the elem
    }
}
*/


// Monomorphization

fn print_all<T>(ts: &[T])
where
    T: ToString,
{
    for t in ts {
        println!("{}", t.to_string());
    }
}

fn smallest<I, T>(ts: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: PartialOrd,
{
    let mut iter = ts.into_iter();
    let mut smallest = iter.next()?;

    for elem in iter {
        if elem < smallest {
            smallest = elem;
        }
    }

    Some(smallest)
}




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
