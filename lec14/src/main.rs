use std::{sync::mpsc::channel, thread};

fn main() {
    let (sender, receiver) = channel();

    thread::scope(|scope| {
        for thread_id in 0..10 {
            let sender = sender.clone();
            scope.spawn(move || {
                sender.send(format!("hello from thread {thread_id}")).unwrap();
            });
        }
        drop(sender);

        scope.spawn(move || {
            while let Ok(value) = receiver.recv() {
                println!("Received value {value}");
            }
        });
    });
}
