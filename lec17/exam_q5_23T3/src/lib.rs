use std::collections::HashMap;

pub fn group_add_items(items: Vec<i32>, grouper: fn(&i32) -> i32) -> HashMap<i32, i32> {
    let mut groupings = HashMap::new();

    for item in items {
        let group = grouper(&item);
        let current_group = groupings.entry(group)
            .or_default();
        *current_group += item;
    }

    groupings
}
