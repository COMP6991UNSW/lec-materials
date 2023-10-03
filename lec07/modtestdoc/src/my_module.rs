pub struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    pub fn new(x: i32, y: i32) -> Foo {
        Foo {
            x,
            y,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
}

pub fn give_int() -> i32 {
    42
}