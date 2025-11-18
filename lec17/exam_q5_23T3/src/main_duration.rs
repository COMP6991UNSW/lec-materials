use std::{collections::{HashMap, LinkedList}, time::Duration};

use exam_q5_lib::group_add_items;

#[derive(PartialEq, Eq, Hash)]
enum DurationType {
    Short,
    Medium,
    Long,
}

impl From<&Duration> for DurationType {
    fn from(duration: &Duration) -> Self {
        use DurationType::*;

        if duration < &Duration::from_secs(1) {
            Short
        } else if duration < &Duration::from_secs(60) {
            Medium
        } else {
            Long
        }
    }
}

fn main() {
    let items = LinkedList::from([
        Duration::from_secs(42),
        Duration::from_millis(500),
        Duration::from_secs(5 * 60),
        Duration::from_secs(59) + Duration::from_millis(999),
        Duration::from_secs(120),
    ]);

    let map: HashMap<DurationType, _> = group_add_items(items, |duration| duration.into());
    assert_eq!(
        map.get(&DurationType::Short),
        Some(&Duration::from_millis(500)),
    );
    assert_eq!(
        map.get(&DurationType::Medium),
        Some(&(Duration::from_secs(42 + 59) + Duration::from_millis(999))),
    );
    assert_eq!(
        map.get(&DurationType::Long),
        Some(&Duration::from_secs(5 * 60 + 120)),
    );
}
