use std::{cell::Cell, rc::Rc};

mod my_mutex;

fn main() {
    use std::thread;
    use my_mutex::MyMutex;

    const N_THREADS:    u64 = 20;
    const N_INCREMENTS: u64 = 1000;
    const EXPECTED:     u64 = N_THREADS * N_INCREMENTS;

    let my_mutex: MyMutex<Rc<u64>> = MyMutex::new(Rc::new(0));

    thread::scope(|scope| {
        for _ in 0..N_THREADS {
            scope.spawn(|| {
                for _ in 0..N_INCREMENTS {
                    let rc = {
                        my_mutex.lock().clone()
                    };

                    let mut rcs = Vec::new();
                    loop {
                        rcs.push(rc.clone());
                    }
                }
            });
        }
    });
}

