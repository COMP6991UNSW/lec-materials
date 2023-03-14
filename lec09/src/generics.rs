pub fn main() {
    dbg!(smallest_two(42, 100));
    dbg!(smallest_two(3.14, 99.9));
    dbg!(smallest_two('a', 'z'));
    dbg!(smallest_two(
        Student { zid: 1234567, name: String::new(), wam: None },
        Student { zid: 1234568, name: String::new(), wam: None },
    ));

    dbg!(smallest(&[42, 100, 3, 1000]));
    dbg!(smallest(&[3.14, 99.9, 10000.0]));
    dbg!(smallest(&['a', 'z', 'q']));
}

#[derive(Debug, PartialEq)]
struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.zid.partial_cmp(&other.zid)
    }
}


/// # Find the smallest element in a set
/// 
/// # Note
/// 
/// If the smallest element appears multiple times,
/// the first one will be returned.
/// 
/// 
/// # Questions for future
/// 
/// Non-ascii passed in?? Unit test??
/// 
fn smallest<I, T>(items: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: PartialOrd,
{
    let mut iter = items.into_iter();
    let mut smallest = iter.next()?;

    for item in iter {
        if item < smallest {
            smallest = item;
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

