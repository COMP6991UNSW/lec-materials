use exam_q5_lib::CountingMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
struct CustomKey<'a> {
    string: &'a str,
    number: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct CustomValue {
    x: i32,
    y: i32,
}

fn main() {
    let keys = [
        CustomKey {
            string: "a",
            number: 1,
        },
        CustomKey {
            string: "a",
            number: 1,
        },
        CustomKey {
            string: "a",
            number: 2,
        },
        CustomKey {
            string: "a",
            number: 3,
        },
        CustomKey {
            string: "a",
            number: 4,
        },
        CustomKey {
            string: "b",
            number: 1,
        },
        CustomKey {
            string: "b",
            number: 2,
        },
        CustomKey {
            string: "b",
            number: 3,
        },
        CustomKey {
            string: "c",
            number: 1,
        },
        CustomKey {
            string: "c",
            number: 1,
        },
        CustomKey {
            string: "c",
            number: 1,
        },
    ];

    let mut map = CountingMap::new();

    for key in keys {
        map.add_to_key(key, CustomValue { x: 1, y: 2 });
    }

    println!("Max: {:?}", map.max_count());
}

impl std::ops::AddAssign for CustomValue {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
