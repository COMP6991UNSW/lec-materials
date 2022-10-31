const N_THREADS: usize = 50;
const N_INCREMENTS: usize = 100000;

#[cfg(any())]
pub mod attempt1 {
    use std::array;

    use super::*;

    static MY_NUMBER: u64 = 0;

    fn thread() {
        for _ in 0..N_INCREMENTS {
            MY_NUMBER += 1;
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

        println!("Final total: {MY_NUMBER} (expected {})\n", N_THREADS * N_INCREMENTS);
    }
}













#[cfg(any())]
pub mod attempt2 {
    use std::array;

    use super::*;

    fn thread(my_number: &mut u64) {
        for _ in 0..N_INCREMENTS {
            *my_number += 1;
        }
    }

    pub fn main() {
        let mut my_number = 0;

        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(|| thread(&mut my_number)));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!("Final total: {my_number} (expected {})\n", N_THREADS * N_INCREMENTS);
    }
}













#[cfg(any())]
pub mod attempt3 {
    use std::{array, cell::Cell};

    use super::*;

    fn thread(my_number: &Cell<u64>) {
        for _ in 0..N_INCREMENTS {
            my_number.set(my_number.get() + 1);
        }
    }

    pub fn main() {
        let mut my_number = Cell::new(0);

        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(|| thread(&my_number)));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!("Final total: {} (expected {})\n", my_number.get(), N_THREADS * N_INCREMENTS);
    }
}













#[cfg(any())]
pub mod attempt4 {
    use std::{array, cell::Cell, sync::{atomic::{AtomicU64, Ordering}, Mutex}};

    use super::*;

    fn thread(my_number: &Mutex<u64>) {
        for _ in 0..N_INCREMENTS {
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        let my_number: Mutex<u64> = Mutex::new(0);
        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(|| thread(&my_number)));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!("Final total: {} (expected {})\n", *my_number.lock().unwrap(), N_THREADS * N_INCREMENTS);
    }
}













#[cfg(all())]
pub mod attempt4fix1 {
    use std::{array, cell::Cell, sync::{atomic::{AtomicU64, Ordering}, Mutex, Arc}};

    use super::*;

    fn thread(my_number: Arc<Mutex<u64>>) {
        for _ in 0..N_INCREMENTS {
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        let my_number: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            let my_number = my_number.clone();
            threads[i] = Some(std::thread::spawn(|| thread(my_number)));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!("Final total: {} (expected {})\n", *my_number.lock().unwrap(), N_THREADS * N_INCREMENTS);
    }
}













#[cfg(all())]
pub mod attempt4fix2 {
    use std::{array, cell::Cell, sync::{atomic::{AtomicU64, Ordering}, Mutex, Arc}};

    use super::*;

    fn thread(my_number: &Mutex<u64>) {
        for _ in 0..N_INCREMENTS {
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        let my_number: Mutex<u64> = Mutex::new(0);

        std::thread::scope(|scope| {
            for i in 0..N_THREADS {
                scope.spawn(|| thread(&my_number));
            }
        });

        println!("Final total: {} (expected {})\n", *my_number.lock().unwrap(), N_THREADS * N_INCREMENTS);
    }
}













#[cfg(all())]
pub mod attempt5 {
    use std::{array, cell::Cell, sync::atomic::{AtomicU64, Ordering}};

    use super::*;

    static MY_NUMBER: AtomicU64 = AtomicU64::new(0);

    fn thread() {
        for _ in 0..N_INCREMENTS {
            MY_NUMBER.fetch_add(1, Ordering::SeqCst);
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

        println!("Final total: {} (expected {})\n", MY_NUMBER.load(Ordering::SeqCst), N_THREADS * N_INCREMENTS);
    }
}
