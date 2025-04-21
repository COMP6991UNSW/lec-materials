use exam_q2_lib::{find_str, most_common};

fn main() {
    let list_of_strs = vec!["hello", "world", "correct", "correct"];

    // Part 1: find_str
    // let search = String::from("correct");
    // let result = find_str(&list_of_strs, &search);
    // drop(search);

    let result = {
        let search = String::from("correct");
        find_str(&list_of_strs, &search)
    };

    println!("find_str: {:?}", result);

    // Part 2: most_common

    // let list_of_strs = vec!["hello", "world", "correct", "correct"];
    // let result = most_common(&list_of_strs);
    // drop(list_of_strs);

    let result = {
        let list_of_strs = vec!["hello", "world", "correct", "correct"];
        most_common(&list_of_strs)
    };

    println!("most_common: {:?}", result);
}
