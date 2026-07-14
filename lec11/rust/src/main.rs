#![allow(unused)]
mod sort;

// declarative macros
// macros-by-example
// macro_rules!

// compared to: procedural macros (proc macros)

macro_rules! add5 {
    ($var:expr) => {
        $var + 5
    };
}

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

fn main() {
    let x = add5!(3) * 2;
    let max = max!(x, 15);
    println!("{x} {max}");
}
