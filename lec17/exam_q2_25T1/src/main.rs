use exam_q2_lib::split_many_1;

fn main() {
    let inputs = vec!["abc:def:ghi".to_string(), "123:456:789".to_string()];
    let splitter = ":";

    // Get the split results.
    let result = split_many_1(&inputs, splitter);

    // Now use the returned slices to make sure they are still valid.
    for (a, sep, b) in result {
        println!("a: {}, sep: {}, b: {}", a, sep, b);
    }

    // Example that drops `splitter` early to test lifetimes
    let result2;
    {
        let split_str = ":";
        result2 = split_many_1(&inputs, split_str);
    } // split_str dropped here
    for (a, sep, b) in result2 {
        // This should fail to compile if lifetimes are incorrect (using dangling `split_str`)
        println!("a: {}, sep: {}, b: {}", a, sep, b);
    }
}
