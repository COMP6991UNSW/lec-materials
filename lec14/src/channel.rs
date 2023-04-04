use std::{sync::mpsc, time::Duration};

pub fn main() {
    let (sender, receiver) = mpsc::channel();

    std::thread::scope(|scope| {
        {
            let sender = sender.clone();
            scope.spawn(move || {
                std::thread::sleep(Duration::from_secs(3));
                println!("Thread {:?} sending message...", std::thread::current().id());
                sender.send(String::from("Hey, you're awesome!")).unwrap();
            });
        }

        scope.spawn(move || {
            std::thread::sleep(Duration::from_secs(5));
            println!("Thread {:?} sending message...", std::thread::current().id());
            sender.send(String::from("You're doing a great job!")).unwrap();
        });

        scope.spawn(move || {
            loop {
                let message = match receiver.recv() {
                    Ok(message) => message,
                    Err(_) => return,
                };
                println!("Thread {:?} received message!", std::thread::current().id());
                println!("I received a message: {message}");
            }
        });
    });
}
