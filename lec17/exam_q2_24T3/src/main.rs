use exam_q2_lib::{find_str, most_common};

fn main() {
    let list_of_strs = vec!["hello", "world", "correct", "correct"];

    // Part 1: find_str
    let result = find_str(&list_of_strs, "correct");

    println!("find_str: {:?}", result);

    // Part 2: most_common

    let result = most_common(&list_of_strs);

    println!("most_common: {:?}", result);
}
