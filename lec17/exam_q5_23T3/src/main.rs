use std::collections::HashMap;

use exam_q5_lib::group_add_items;

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7];

    let map = group_add_items(items, |x| x % 3);
    assert_eq!(
        map,
        HashMap::from([
            (0, 9),
            (1, 12),
            (2, 7),
        ]),
    );
}
