use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        })
    }).collect();

    drop(counter);

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Final: {}", *counter.lock().unwrap());
}
