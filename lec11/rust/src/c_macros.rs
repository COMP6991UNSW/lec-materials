macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

macro_rules! up_to {
    ($i:ident, $n:expr, $body:block) => {
        for $i in 0..$n $body
    };
}

pub fn main() {
    let x = max!(42, 123);
    println!("{x}");

    up_to!(foo, 10, {
        println!("{foo}");
    });
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
