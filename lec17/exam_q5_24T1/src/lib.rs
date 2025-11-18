use std::{sync::mpsc::channel, thread};

pub fn parallel_reduce(items: Vec<i32>, identity: i32, reducer: fn(i32, i32) -> i32) -> i32 {
    let (a_send, a_recv) = channel();
    let (b_send, b_recv) = channel();
    let (res_send, res_recv) = channel();

    thread::scope(|scope| {
        for chan in [a_recv, b_recv] {
            let reducer = &reducer;
            let identity = identity.clone();
            let res_send = res_send.clone();

            scope.spawn(move || {
                let mut acc = identity;

                while let Ok(elem) = chan.recv() {
                    acc = reducer(acc, elem);
                }

                res_send.send(acc).unwrap();
            });
        }

        let mut is_a = true;
        for item in items {
            let sender = if is_a { &a_send } else { &b_send };
            is_a = !is_a;
            sender.send(item).unwrap();
        }

        drop(a_send);
        drop(b_send);

        let a = res_recv.recv().unwrap();
        let b = res_recv.recv().unwrap();
        reducer(a, b)
    })
}
