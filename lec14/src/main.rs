use std::{sync::mpsc, thread};

use rayon::iter::{ParallelIterator, IntoParallelIterator};

fn main() {
    let answer: f64 = (0_u64..100_000_000_000)
        .into_par_iter()
        .map(|x| x as f64)
        .sum();

    println!("{answer}");
}


/*
    let (send, recv) = mpsc::channel();

    let mut thread_handles = vec![];
    for i in 0..5 {
        let send = send.clone();

        let handle = thread::spawn(move || {
            send.send(format!("hello from thread {i}")).unwrap();
        });
        thread_handles.push(handle);
    }
    drop(send);

    thread::spawn(move || {
        while let Ok(message) = recv.recv() {
            println!("I received a message: {message}");
        }
    }).join().unwrap();

    for handle in thread_handles {
        handle.join().unwrap();
    }
}




























    */
