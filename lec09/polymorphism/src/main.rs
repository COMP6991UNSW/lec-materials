#![allow(unused)]

pub fn main() {
    dbg!(smallest_char('a', 'b'));
    dbg!(smallest_int(3, 2));
    dbg!(smallest_float(3.5, 3.7));
}

fn smallest_char(x: char, y: char) {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_float(x: f32, y: f32) {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_float(x: i32, y: i32) {
    if x < y {
        x
    } else {
        y
    }
}
