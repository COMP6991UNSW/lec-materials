#![allow(unused)]

use std::{time::Duration, thread};

mod data_race;
mod unsafe_race;


//     <-------- time ---------->
// T1 [------      -------     --]
// T2 [      ------       -----  ]

fn concurrency_ex() {
    let thread1 = std::thread::spawn(task1);
    let thread2 = std::thread::spawn(task2);

    thread1.join().unwrap();
    thread2.join().unwrap();

    // std::thread::spawn(|| {
    //     task2();
    // });

    // task1();
}

fn task1() {
    loop {
        println!("hello");
        std::thread::sleep(Duration::from_secs(3));
    }
}

fn task2() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {input}");
    }
}   

fn main() {
    concurrency_ex();

    // data_race::attempt1::main();
    // data_race::attempt2::main();
    // data_race::attempt3::main();
    // data_race::attempt4::main();
    // data_race::attempt4fix1::main();
    // data_race::attempt4fix2::main();
    // data_race::attempt5::main();
    // unsafe_race::main();
}
