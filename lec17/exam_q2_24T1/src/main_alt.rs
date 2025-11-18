#![allow(unused)]

use exam_q2_lib::{print, first, StringSplitter, split};

fn main() {
    // Test "print"

    let b: i32 = 10;
    print(&b);

    // Test "first"

    {
        let a: i32 = 5;
        first(&a, &b);
    }

    // Test "split"

    let pattern = String::from("l");
    let pattern_ref = {
        let text = String::from("hello");
        let StringSplitter { text: _, pattern: new_pattern } = split(text.as_str(), pattern.as_str());
        new_pattern
    };

    assert_eq!(pattern_ref, pattern.as_str());
}
