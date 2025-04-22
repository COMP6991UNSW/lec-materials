fn main() {
    spam_text_2("Hello, world!");
}

use std::thread;

fn spam_text_1(message: &str) {
    let mut handles = Vec::new();

    for _ in 0..5 {
        let message_owned = message.to_string();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                println!("{message_owned}");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn spam_text_2(message: &str) {
    thread::scope(|scope| {
        for _ in 0..5 {
            scope.spawn(|| {
                for _ in 0..20 {
                    println!("{message}");
                }
            });
        }
    });
}


fn spam_text_3(message: &str) {
    let message: &'static str = Box::leak(message.to_string().into_boxed_str());

    let mut handles = Vec::new();

    for _ in 0..5 {
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                println!("{message}");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
