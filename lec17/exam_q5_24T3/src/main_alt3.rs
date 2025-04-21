use exam_q5_lib::DedupNotifier;

#[derive(Debug, Clone)]
struct Event {
    time: i64,
    event_id: i64,
    text: String,
}

fn main() {
    let event1 = Event {
        time: 944784000,
        event_id: 1,
        text: "Happy Birthday Tom".to_string(),
    };
    let event2 = Event {
        time: 971481600,
        event_id: 2,
        text: "Happy Birthday Zac".to_string(),
    };

    let (snd1, rcv1) = std::sync::mpsc::channel();
    let (snd2, rcv2) = std::sync::mpsc::channel();

    let mut dn = DedupNotifier::new(|event: Event| event.time + event.event_id);
    println!("Add first listener and test.");
    dn.add_listener(snd1);
    dn.notify(event1.clone());
    println!("Add second listener and test different word.");
    dn.add_listener(snd2);
    dn.notify(event2);
    println!("Now test the same word as the first time.");
    dn.notify(event1);

    println!("1/1: {:?}", rcv1.recv().unwrap().text);
    println!("1/2: {:?}", rcv1.recv().unwrap().text);

    println!("2/1: {:?}", rcv2.recv().unwrap().text);
    println!("2/2: {:?}", rcv2.recv().unwrap().text);

    println!("Done!")
}
