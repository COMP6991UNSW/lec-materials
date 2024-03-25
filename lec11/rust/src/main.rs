#![allow(unused)]

mod sort;

// How does #[derive(...)] work?
#[derive(Debug, PartialEq, PartialOrd)]
struct Student {
    zid: u32,
    name: String,
}

macro_rules! max {
    ($a:expr, $b:expr $(,)?) => {
        if $a > $b { $a } else { $b }
    };
}

macro_rules! up_to {
    ($i:ident, $n:expr, $body:block) => {
        for $i in 0..$n $body
    };
}

macro_rules! my_vec {
    ($($elem:expr),* $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($elem);
            )*
            vec
        }
    };
}

macro_rules! cfor {
    // TODO
    () => {};
}

fn main() {
    let x = max!(10, 42);
    println!("{x}");

    let x = max!(
        Student {
            zid: 7654321,
            name: String::from("Irfan"),
        },
        Student {
            zid: 1234567,
            name: String::from("Brian"),
        },
    );

    up_to!(i, 10, {
        println!{"i={i}"};
    });

    let v = my_vec![1, 2, 3];
    println!("{v:?}");
    
    /*
    cfor! {
        for (let mut i = 0; i < 42; i += 1) {
            println!("i = {i}");
        }
    }
    */

    // sort::test_sort();
}
