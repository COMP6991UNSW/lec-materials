use std::{sync::mpsc, thread, time::Duration};

enum Message {
    HeyThisHappened { associated_data: String },
    ThisOtherEventHappened { zid: String, wam: Option<f64>, },
    ActuallyMaybeExitNow,
}

pub fn main() {
    let (sender, receiver) = mpsc::channel::<String>();

    {
        let sender = sender.clone();

        std::thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            sender.send(String::from("Here's that work you asked for!"))
                .unwrap();
        });
    }

    std::thread::spawn(move || {
        thread::sleep(Duration::from_secs(4));
        sender.send(String::from("And I'm done too!!"))
            .unwrap();
    });

    for _ in 0..2 {
        let message = receiver.recv().unwrap();
        println!("Main received a message: {message}");
    }
}
