use std::{collections::HashMap, borrow::Cow};

use exam_q5_lib::group_add_items;
use Verdict::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Verdict {
    Naughty,
    Nice,
}

impl Verdict {
    fn flip(&mut self) {

        *self = match self {
            Naughty => Nice,
            Nice => Naughty,
        }
    }
}

fn main() {
    let staff = [
        Cow::from("Zac\n"),
        Cow::from("Tom\n"),
        Cow::from("Andrew\n"),
        Cow::from("Shrey\n"),
    ];

    let mut verdict = Nice;

    let map = group_add_items(staff, |_| {
        verdict.flip();
        verdict
    });

    assert_eq!(
        map,
        HashMap::from([
            (Naughty, Cow::from("Zac\nAndrew\n")),
            (Nice, Cow::from("Tom\nShrey\n")),
        ]),
    );
}
