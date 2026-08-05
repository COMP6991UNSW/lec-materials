use exam_q5_25T3_lib::Processor;

#[derive(Debug)]
struct MapOnly(i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Filterable(i32);

fn main() {
    let processor = Processor::new(vec![MapOnly(1), MapOnly(2), MapOnly(3)]);
    let mapped = processor.map_all(|item| item.0 * 2);
    println!("Mapped: {:?}", mapped);

    let processor = Processor::new(vec![
        Filterable(1),
        Filterable(2),
        Filterable(2),
        Filterable(3),
    ]);
    let filtered = processor.filter_dedup(|item| item.0 > 1);
    println!("Filtered unique: {:?}", filtered);

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Sortable(i32);

    let processor = Processor::new(vec![Sortable(3), Sortable(1), Sortable(2)]);
    let sorted = processor.into_sorted(|v| v);
    println!("Sorted: {:?}", sorted);
}
