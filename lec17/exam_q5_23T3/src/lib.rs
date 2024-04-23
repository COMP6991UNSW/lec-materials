use std::{collections::HashMap, hash::Hash, ops::AddAssign};

pub fn group_add_items<I, F, Item, Group>(
    items: I,
    mut grouper: F,
) -> HashMap<Group, Item>
where
    I: IntoIterator<Item = Item>,
    F: FnMut(&Item) -> Group,
    Group: Eq + Hash,
    Item: Default + AddAssign,
{
    let mut groupings = HashMap::new();

    for item in items {
        let group = grouper(&item);
        let current_group = groupings.entry(group)
            .or_default();
        *current_group += item;
    }

    groupings
}
