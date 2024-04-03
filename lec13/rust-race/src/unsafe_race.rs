const N_THREADS: usize = 50;
const N_INCREMENTS: usize = 100000;
const EMPTY_JOIN_HANDLE: Option<JoinHandle<()>> = None;

use std::{array, thread::JoinHandle};

use super::*;

static mut MY_NUMBER: u64 = 0;

fn thread() {
    for _ in 0..N_INCREMENTS {
        // Safety: This is exactly why what I'm doing here
        // is safe in ALL contexts ...
        unsafe { MY_NUMBER += 1 };
    }
}

pub fn main() {
    let mut threads = [EMPTY_JOIN_HANDLE; N_THREADS];

    for i in 0..N_THREADS {
        threads[i] = Some(std::thread::spawn(thread));
    }
    for i in 0..N_THREADS {
        threads[i].take().unwrap().join();
    }

    println!("(Unsafe) Final total: {} (expected {})\n", unsafe { MY_NUMBER }, N_THREADS * N_INCREMENTS);
}
