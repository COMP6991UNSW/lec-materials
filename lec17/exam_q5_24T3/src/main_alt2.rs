use exam_q5_lib::DedupNotifier;

#[derive(Debug, Clone, Copy)]
struct Event {
    time: i64,
    event_id: i64,
}

fn hasher(event: Event) -> i64 {
    event.time + event.event_id
}

fn main() {
    let event1 = Event {
        time: 944784000,
        event_id: 1,
    };
    let event2 = Event {
        time: 971481600,
        event_id: 2,
    };

    let (snd1, rcv1) = std::sync::mpsc::channel();
    let (snd2, rcv2) = std::sync::mpsc::channel();

    let mut dn = DedupNotifier::new(hasher);
    println!("Add first listener and test.");
    dn.add_listener(snd1);
    dn.notify(event1.clone());
    println!("Add second listener and test different word.");
    dn.add_listener(snd2);
    dn.notify(event2);
    println!("Now test the same word as the first time.");
    dn.notify(event1);

    println!("1/1: {:?}", rcv1.recv().unwrap());
    println!("1/2: {:?}", rcv1.recv().unwrap());

    println!("2/1: {:?}", rcv2.recv().unwrap());
    println!("2/2: {:?}", rcv2.recv().unwrap());

    println!("Done!")
}
