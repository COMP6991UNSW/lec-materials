#![allow(unused)]

use std::{sync::mpsc::channel, thread::{self, sleep}, time::Duration};

mod data_race;
mod unsafe_race;

fn concurrency() {
    let (send, recv) = channel();

    thread::scope(|scope| {
        for thread_id in 0..10 {
            let send = send.clone();
            scope.spawn(move || {
                // make it do some work
                send.send(thread_id).unwrap();
                send.send(thread_id).unwrap();
                send.send(thread_id).unwrap();
            });
        }
    });

    for item in recv {
        println!("Received: {item}");
    }


    // let handle = thread::spawn(move || {
    //     println!("Thread has been spawned!");
    //     let message = recv.recv().unwrap();

    //     println!("Received message from main: {message}");
    // });

    // sleep(Duration::from_secs(3));
    // send.send(String::from("hello thread!")).unwrap();
    // // ...

    // handle.join().unwrap();
}




fn main() {
    concurrency();


    loop {}


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

    handle1.join();
    handle2.join();
}

fn task1() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        println!("Echo: {s}");
    }
}

fn task2() {
    loop {
        println!("Hello!");
        sleep(Duration::from_secs(1));
    }
}

// fn concurrency() {
//     let mut my_string1 = String::from("hi"); // capture move
//     let mut my_string2 = String::from("hi"); // capture by borrow
//     let mut my_string3 = String::from("hi"); // capture a clone
// 
//     {
//         let my_string2 = &my_string2;
//         let my_string3 = my_string3.clone();
// 
//         thread::scope(|scope| {
//             scope.spawn(|| {
//                 println!("hello from scoped thread {my_string2}!");
//             });
//             scope.spawn(|| {
//                 println!("hello from scoped thread!");
//             });
//             scope.spawn(|| {
//                 println!("hello from scoped thread!");
//             });
//         });
// 
//         let handle = thread::spawn(move || {
//             println!("hello from thread, string is {}", my_string1);
//             // println!("hello from thread, string is {}", my_string2);
//             println!("hello from thread, string is {}", my_string3);
// 
//             return 42;
//         });
// 
//         let x = handle.join().unwrap();
//     }
// 
//     println!("hello from main thread! string is {}", my_string3);
// }
// 
// 
