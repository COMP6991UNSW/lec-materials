#![allow(unused)]

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
    ($name:ident, $n:expr, $body:block) => {
        for $name in 0..$n $body
    };
}

macro_rules! my_vec {
    ($($item:expr),* $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($item);
            )*
            vec
        }
    };
}

macro_rules! cfor {
    (for ($init:stmt ; $cond:expr ; $step:stmt) $body:block) => {
        $init;
        while $cond {
            $body

            $step
        }
    };
}

fn main() {
    up_to!(i, 10, {
        println!{"i={i}"};
    });

    let x = max!(10, 42);
    println!("{x}");

    let v = my_vec![1, 2, 3];

    println!("{v:?}");

    
    cfor! {
        for (let mut i = 0; i < 42; i += 1) {
            println!("i = {i}");
        }
    }

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

    println!("{x:?}");
}




mod sort {
    use paste::paste;

    macro_rules! declare_sort {
        ($prefix:ident, $type:ty) => {
            paste! {
                use std::cmp::Ordering;

                fn [<_declare_sort_ $prefix _compare>](x: &$type, y: &$type) -> Ordering {
                    if x > y {
                        Ordering::Greater
                    } else if x < y {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }

                fn [<$prefix _sort>](slice: &mut [$type]) {
                    slice.sort();
                } 
            }
        }
    }

    declare_sort!(int, i32);

    pub fn test_sort() {
        let mut xs = [6, 1, 3, 9, 10, 5, 2, 4, 8, 7];
        int_sort(&mut xs);

        println!("Sorted: {xs:?}");
    }
}
