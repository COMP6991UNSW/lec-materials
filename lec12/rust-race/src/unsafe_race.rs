const N_THREADS: usize = 50;
const N_INCREMENTS: usize = 100000;

use std::array;

use super::*;

static mut MY_NUMBER: u64 = 0;

fn thread() {
    for _ in 0..N_INCREMENTS {
        unsafe { MY_NUMBER += 1 };
    }
}

pub fn main() {
    let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

    for i in 0..N_THREADS {
        threads[i] = Some(std::thread::spawn(thread));
    }
    for i in 0..N_THREADS {
        threads[i].take().unwrap().join();
    }

    println!("(Unsafe) Final total: {} (expected {})\n", unsafe { MY_NUMBER }, N_THREADS * N_INCREMENTS);
}
