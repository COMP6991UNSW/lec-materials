#![allow(unused)]

use std::{io::stdin, sync::mpsc::channel, thread::{self, sleep}, time::Duration};

mod data_race;
mod unsafe_race;


// CPU cores: how many things your computer can do *at the same time* (in parallel)
//
// T1 [----------                                     ------
// T2 [          --------
// T3 [                              -----------------                   ------------
// T4 [                  ------------
// T5 [                                                     -------------
// ==================================== time -> =======================================

// Waiting for some resource or time: blocking (your thread is blocked)

fn main() {
    // concurrency_example_2();

    // data_race::attempt1::main();
    // data_race::attempt2::main();
    // data_race::attempt3::main();
    // data_race::attempt5::main();
    // data_race::attempt4fix1::main();
    // data_race::attempt4fix2::main();
    // data_race::attempt5::main();

    // unsafe_race::main();
}

fn concurrency_example_2() {
    let jh1 = thread::spawn(task_1);
    let jh2 = thread::spawn(task_2);

    jh1.join();
    jh2.join();
}

fn task_1() {
    loop {
        println!("hello world!");
        thread::sleep(Duration::from_secs(1));
    }
}

fn task_2() {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        println!("you entered: {line}");
    }
}






/*

fn first_concurrency() {
    my_thread_spawn(|| println!("hello!"));

    let mut join_handles = vec![];
    join_handles.push(thread::spawn(concurrency_example));
    join_handles.push(thread::spawn(|| task_1()));
    join_handles.push(thread::spawn(|| {
        task_2();
    }));

    for join_handle in join_handles {
        join_handle.join();
    }
}

fn my_thread_spawn<F: FnOnce()>(f: F) {
    f();
}

fn concurrency_example() {
    let x = 42;
    let y = x * 3;
    let z = y - 5;
    println!("{z}");
    println!("{z}");
    println!("{z}");
    println!("{z}");
    println!("{z}");
}

fn task_1() {
    thread::spawn(|| {
        thread::spawn(|| {

        });
    });

    for _ in 0..5 {
        println!("hello world!");
    }
}

fn task_2() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    println!("you entered: {line}");
}

*/
