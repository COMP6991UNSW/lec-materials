macro_rules! map {
    ($($tokens:tt)*) => {
        {
            let mut map = ::std::collections::BTreeMap::new();

            map_inner!(map; $($tokens)*);
            
            map
        }
    };
}

// recursive tt munching
macro_rules! map_inner {
    ($map:expr; ) => {

    };
    ($map:expr; $key:expr => $value:expr,$($rest:tt)*) => {
        $map.insert($key, $value);

        map_inner!($map; $($rest)*);
    };
    ($map:expr; $key:expr => ref $ref_key:expr,$($rest:tt)*) => {
        let val = *$map.get(&$ref_key).unwrap();
        $map.insert($key, val);

        map_inner!($map; $($rest)*);
    };
}

fn main() {
    let map = map![
        0 => "a",
        1 => ref 0,
        2 => "b",
        3 => ref 1,
    ];

    // The code above expands into:
    // let map = {
    //     let mut map = ::std::collections::BTreeMap::new();

    //     map.insert(0, "a");
    //     map.insert(1, "b");
    //     map.insert(2, "c");

    //     map
    // };

    // Prints: {0: "a", 1: "b", 2: "c"}
    println!("{map:?}");
}
