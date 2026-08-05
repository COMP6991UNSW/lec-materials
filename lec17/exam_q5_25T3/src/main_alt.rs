use exam_q5_25T3_lib::Processor;

fn main() {
    let mut count = 0;
    let processor = Processor::new(vec![1, 2, 3, 4, 5]);
    let mapped = processor.map_all(|x| {
        count += 1;
        x * count
    });
    println!("Mapped: {:?}", mapped);

    let mut call_count = 0;
    let processor = Processor::new(vec![1, 2, 2, 3, 4, 4, 4]);
    let filtered = processor.filter_dedup(|x| {
        call_count += 1;
        *x % 2 == 0
    });
    println!("Filtered unique: {:?} (predicate called {} times)", filtered, call_count);

    let consumed = vec![100, 200];
    let processor = Processor::new(vec![5, 3, 1, 4, 2]);
    let sorted = processor.into_sorted(move |mut v| {
        v.extend(consumed);
        v
    });
    println!("Sorted: {:?}", sorted);
}
