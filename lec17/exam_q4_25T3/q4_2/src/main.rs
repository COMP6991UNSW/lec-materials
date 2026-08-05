use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    
    let handle = {
        let data = data.clone();

        thread::spawn(move || {
            println!("{data:?}");
        })
    };
    
    handle.join().unwrap();
    // ..
}
