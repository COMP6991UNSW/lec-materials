pub fn function_from_a() -> i32 {
    42
}

pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new(x: i32) -> Option<Self> {
        if x < 0 || x > 100 {
            None
        } else {
            Some(
                Self {
                    x,
                }
            )
        }
    }

    pub fn foo(&self) {
        println!("Foo has x={}", self.x);
    }
}
