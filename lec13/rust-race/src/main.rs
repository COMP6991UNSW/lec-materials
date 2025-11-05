#![allow(unused)]

use std::{io::stdin, sync::mpsc::channel, task, thread::{self, sleep}, time::Duration};

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

    unsafe_race::main();
}

fn concurrency_example() {
    // thing 1: print "hello" to the terminal, once per second
    // thing 2: echo user input back to the user

    let mut value_handle = std::thread::spawn(produces_value);

    let handle1 = std::thread::spawn(task1);
    let handle2 = std::thread::spawn(task2);

    loop {
        println!("waiting for thread's result...");
        let result = value_handle.join();

        match result {
            Ok(result) => {
                println!("got the result!");
                dbg!(result);
                break;
            }
            Err(e) => {
                println!("issue with thread!!! trying again!");
                value_handle = std::thread::spawn(produces_value);
            }
        }
    }

    dbg!(handle1.thread().name());
    dbg!(handle2.thread().name());

    handle1.join();
    handle2.join();
}

fn produces_value() -> i32 {
    std::thread::sleep(Duration::from_secs(3));

    if true {
        panic!("oh no!!!");
    }

    42
}

fn task1() {
    loop {
        println!("hello");
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn task2() {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        print!("Input: {line}");
    }
}

fn task1and2() {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        print!("Input: {line}");

        println!("hello");
        std::thread::sleep(Duration::from_secs(1));
    }
}
