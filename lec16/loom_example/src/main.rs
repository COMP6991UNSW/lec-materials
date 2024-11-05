#[cfg(loom)]
use loom::{thread, sync::atomic::AtomicUsize};

#[cfg(not(loom))]
use std::{thread, sync::atomic::AtomicUsize};

use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;


fn main() {
    let value = some_concurrent_logic();
    println!("{value}");
}

fn some_concurrent_logic() -> usize {
    let v1 = Arc::new(AtomicUsize::new(0));
    let v2 = v1.clone();

    thread::spawn(move || {
        v1.store(1, SeqCst);
    });

    v2.load(SeqCst)
}


#[cfg(test)]
mod tests {
    use crate::some_concurrent_logic;

    #[test]
    fn test_with_loom() {
        loom::model(|| {
            let value = some_concurrent_logic();
            assert_eq!(0, value)
        });
    }
}
