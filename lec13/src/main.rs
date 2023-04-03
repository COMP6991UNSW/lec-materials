#![allow(unused)]

use std::{time::Duration, thread};

mod data_race;
mod unsafe_race;

//         vvvvvvvvvvvvvv
// idea 1 [--------------.        .---------------.              ]
// idea 2 [              .--------.               .--------------]


fn some_concurrency() {
    std::thread::spawn(|| {
        // idea 1
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("You inputted: {input}");
    });

    // idea 2
    loop {
        println!("Hello!");
        std::thread::sleep(Duration::from_secs(1));
    }


}

fn main() {
    // some_concurrency();
    // data_race::attempt1::main();
    // data_race::attempt2::main();
    // data_race::attempt3::main();
    // data_race::attempt4::main();
    data_race::attempt4fix1::main();
    // data_race::attempt4fix2::main();
    // data_race::attempt5::main();
    // unsafe_race::main();
}
