use std::collections::HashMap;

use exam_q5_lib::group_add_items;

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7];

    let map = group_add_items(items, |x| x % 2 == 0);
    assert_eq!(
        map,
        HashMap::from([
            (false, 1 + 3 + 5 + 7),
            (true,  2 + 4 + 6),
        ]),
    );
}
