#![allow(unused)]

use std::{sync::mpsc::channel, thread::{self, sleep}, time::Duration};

mod data_race;
mod unsafe_race;

fn main() {
    // concurrency_example();

    // data_race::attempt1::main();
    // data_race::attempt2::main();
    // data_race::attempt3::main();
    // data_race::attempt5::main();
    // data_race::attempt4fix1::main();
    // data_race::attempt4fix2::main();
    // data_race::attempt5::main();

    // unsafe_race::main();
}

fn concurrency_example() {
    let handle1 = std::thread::spawn(task1);
    let handle2 = std::thread::spawn(task2);

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn task1() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        println!("you said: {input}");
    }
}

fn task2() {
    loop {
        println!("hello");
        std::thread::sleep(Duration::from_secs(2));
    }
}
