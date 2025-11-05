use std::{sync::mpsc::channel, thread};

fn main() {
    let (send, recv) = std::sync::mpsc::channel();

    thread::scope(|scope| {
        for n in 0..5 {
            let send = send.clone();

            scope.spawn(move || {
                send.send(n).unwrap();
            });
        }
    });
    drop(send);

    while let Ok(message) = recv.recv() {
        println!("Got message: {message}");
    }
}
