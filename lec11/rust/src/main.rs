#![allow(unused)]

mod sort;

macro_rules! add5 {
    ($input:expr) => {
        $input + 5
    };
}

macro_rules! mul7 {
    ($input:expr) => {
        $input * 7
    };
}


fn main() {
    let x = add5!(2 + 2) * 5;
    let y = mul7!(2 + 2);
    println!("{x} {y}");

    let std_vec = vec![
        1,
        2,
        3,
    ];
    println!("{std_vec:?}");

    // let std_vec = {
    //     let mut v = Vec::new();
    //     v.push(1);
    //     v.push(2);
    //     v.push(3);
    //     v
    // };

    let my_vec = my_vec![
        1,
        2,
        3,
        4,
        5
    ];
    
    println!("{my_vec:?}");
}


macro_rules! my_vec {
    ($($item:expr),*) => {
        {
            let mut v = Vec::new();
            
            $(
                v.push($item);
            )*

            v
        }
    };
    // ($($item:expr),*,) => {
    //     {
    //         let mut v = Vec::new();
            
    //         $(
    //             v.push($item);
    //         )*

    //         v
    //     }
    // };
}
use my_vec;
