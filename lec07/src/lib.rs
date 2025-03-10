#![allow(unused)]

pub mod point;
mod another_file;
mod stuff;
mod stuff2;

pub fn add(point: point::Point) -> i32 {
    point.x + point.y
}

impl point::Point {
    // fn hello(&self) {
    //     println!("hello from {} {}", self.x, self.y);
    // }
}


mod my_module {
    pub struct Point {
        x: i32,
        y: i32,
    }

    pub(crate) enum Foo {
        A,
        B(i32),
        C { x: i32, y: f64 },
    }

    impl Point {
        pub fn x(&self) -> i32 {
            self.x
        }

        pub fn y(&self) -> i32 {
            self.y
        }

        pub fn x_mut(&mut self) -> &mut i32 {
            &mut self.x
        }

        // pub fn set_x(&mut self, x: i32) {
        //     self.x = x;
        // }
    }

    fn hello() {
        println!("hello!");
    }
}
