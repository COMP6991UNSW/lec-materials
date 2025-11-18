use exam_q5_lib::CountingMap;

fn main() {
    let strings = [
        "a", "b", "c", //
        "a", "b", "a", //
        "c", "b", "a",
    ];

    let mut map = CountingMap::new();

    for string in strings {
        map.add_to_key(String::from(string), 1);
    }

    println!("Max: {:?}", map.max_count());
    println!("B count: {}", map.get_mut(String::from("b")));
    println!("C count: {}", map.get_mut(String::from("c")));
}
