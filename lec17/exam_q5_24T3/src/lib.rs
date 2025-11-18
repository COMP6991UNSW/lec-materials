use std::sync::mpsc::Sender;

/// The Event type represents a pair of time and event_id.
pub type Event = (u64, u64);

struct Listener {
    sender: Sender<Event>,
    prev_hashes: Vec<Event>,
}

pub struct DedupNotifier {
    listeners: Vec<Listener>,
    hasher: fn(Event) -> Event,
}

impl DedupNotifier {
    pub fn new(hasher: fn(Event) -> Event) -> Self {
        Self {
            hasher: hasher,
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, notifier: Sender<Event>) {
        self.listeners.push(Listener {
            sender: notifier,
            prev_hashes: Vec::new(),
        });
    }

    pub fn notify(&mut self, message: Event) {
        let hash = (self.hasher)(message.clone());
        self.listeners.iter_mut().for_each(|listener| {
            if listener.prev_hashes.contains(&hash) {
                return;
            } else {
                listener.prev_hashes.push(hash);
            }
            listener.sender.send(message.clone()).unwrap();
        });
    }
}
