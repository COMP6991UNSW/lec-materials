#![allow(unused)]

mod c_macros;
mod rust_macros;

macro_rules! count_stuff {
    () => {0};
    ($($b:tt $c:tt)*) => {(2 * count_stuff!($($b)*)) };
    ($a:tt $($b:tt $c:tt)*) => {1 + (2 * count_stuff!($($b)*)) };
}

fn main() {
    let one = count_stuff!(a);
    let some = count_stuff!(a b c d e f g h i j k l m n o);
    let more = count_stuff!(a b c d e f g h i j k l m n o p);

    c_macros::main();
    rust_macros::main();
}
