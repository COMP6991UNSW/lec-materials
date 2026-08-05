use exam_q5_25T3_lib::Processor;

fn main() {
    let processor = Processor::new(vec![
        "hello".to_string(),
        "world".to_string(),
    ]);
    let mapped = processor.map_all(|s| s.to_uppercase());
    println!("Mapped: {:?}", mapped);

    let processor = Processor::new(vec![
        "hello".to_string(),
        "world".to_string(),
        "hello".to_string(),
    ]);
    let filtered = processor.filter_dedup(|s| s.len() > 3);
    println!("Filtered unique: {:?}", filtered);

    let processor = Processor::new(vec![
        "world".to_string(),
        "hello".to_string(),
        "goodbye".to_string(),
    ]);
    let sorted = processor.into_sorted(|v| v);
    println!("Sorted: {:?}", sorted);
}
