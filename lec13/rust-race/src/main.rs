#![allow(unused)]

use std::{thread::{self, sleep}, time::Duration};

mod data_race;
mod unsafe_race;

fn main() {
    // concurrency_example();

    // data_race::attempt1::main();
    // data_race::attempt2::main();
    // data_race::attempt3::main();
    // data_race::attempt5::main();
    // data_race::attempt4fix1::main();
    data_race::attempt4fix2::main();
    // data_race::attempt5::main();
    // unsafe_race::main();
}

fn concurrency_example() {
    let handle_1 = thread::spawn(task_1);
    let handle_2 = thread::spawn(task_2);

    handle_1.join();
    handle_2.join();
}

// Print "hello" every 3 seconds, forever
fn task_1() {
    loop {
        println!("hello");
        sleep(Duration::from_secs(3));
    }
}

// Echo user input back out to them, forever
// > hello
// You entered: hello
// > world
// You entered: world
fn task_2() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);

        println!("You entered: {line}");
    }
}
