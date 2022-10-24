use std::fmt::Display;

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    }
}

pub fn test_max() {
    fn print_and_return<T: Display>(x: T) -> T {
        println!("{x}");
        x
    }

    dbg!(max!(print_and_return(42), print_and_return(100)));
}
