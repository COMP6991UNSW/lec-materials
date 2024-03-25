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
