use exam_q2_lib::{identity, swap, sort_references};

fn main() {
    // Test "identity"

    let a: i32 = 15;

    assert_eq!(identity(&a), &a);

    // Test "swap"

    let a_ref = {
        let b: i32 = 10;
        let (new_a, _new_b) = swap(&b, &a);
        new_a
    };

    assert_eq!(a_ref, &a);

    // Test "sort_references"

    let b: i32 = 10;
    let (_new_b, new_a) = sort_references(&a, &b);

    assert_eq!(new_a, &a);
}
