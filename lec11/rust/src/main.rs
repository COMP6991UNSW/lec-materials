#![allow(unused)]

mod sort;

// #define ADD5(x) x + 5

macro_rules! add5 {
    ($x:expr) => {
        $x + 5
    };
}

macro_rules! mul7 {
    ($x:expr) => {
        $x * 7
    };
}

macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
}

macro_rules! my_vec {
    [] => {
        Vec::new()
    };
    [$($item:expr),+ $(,)?] => {
        {
            let mut v = Vec::new();
            $(
                v.push($item);
            )+
            v
        }
    }
}

fn main() {
    let x = mul7!(add5!{2});
    let y = max!(dbg!(x), dbg![42]);
    println!("{y}");

    let std_vec = vec!{
        1,
        2,
        3,
    };
    println!("{std_vec:?}");

    let my_vec = my_vec![
        1,
        2,
        3,
        4,
        5,
    ];

    println!("{my_vec:?}");
}
