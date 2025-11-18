#![allow(unused)]

use exam_q2_lib::{print, first, StringSplitter, split};

fn main() {
    // Test "print"

    let a: i32 = 5;
    print(&a);

    // Test "first"

    let a_ref = {
        let b: i32 = 10;
        let new_a = first(&a, &b);
        new_a
    };
    assert_eq!(a_ref, &a);

    // Test "split"

    let text = String::from("hello");
    let text_ref = {
        let pattern = String::from("l");
        let StringSplitter { text: new_text, pattern: _ } = split(text.as_str(), pattern.as_str());
        new_text
    };

    assert_eq!(text_ref, text.as_str());
}
