use exam_q5_25T3_lib::Processor;

struct DropLogger(&'static str);

impl Drop for DropLogger {
    fn drop(&mut self) {
        println!("Dropped: {}", self.0);
    }
}

fn main() {
    let mut multiplier = 1;
    let processor = Processor::new(vec![1, 2, 3]);
    let mapped = processor.map_all(|x| {
        multiplier *= 2;
        x * multiplier
    });
    println!("Mapped: {:?}", mapped);

    let mut threshold = 0;
    let processor = Processor::new(vec![1, 2, 2, 3, 4, 4]);
    let filtered = processor.filter_dedup(|x| {
        threshold += 1;
        *x >= threshold
    });
    println!("Filtered unique: {:?}", filtered);

    let logger = DropLogger("FnOnce consumed");
    let processor = Processor::new(vec![5, 3, 1, 4, 2]);
    let sorted = processor.into_sorted(move |v| {
        drop(logger);
        v
    });
    println!("Sorted: {:?}", sorted);
}
