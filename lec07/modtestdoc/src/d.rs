pub fn foo() {

}

pub struct Foo {
    pub x: i32,
    y: String,
}

impl Foo {
    pub fn new() -> Self {
        Foo {
            x: 42,
            y: String::from("foo"),
        }
    }

    pub fn y_len(&self) -> usize {
        self.y.len()
    }
}
