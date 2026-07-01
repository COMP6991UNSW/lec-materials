#![allow(unused)]

use std::f64;

fn main() {
    dbg!(
        sum_areas(vec![
            &Square { length: 3.0 },
            &Square { length: 2.0 },
            &Rectangle {
                height: 2.0,
                length: 3.0,
            },
            &Circle {
                radius: 1.0,
            },
        ])
    );
}

fn sum_areas(shapes: Vec<&dyn Shape>) -> f64 {
    let mut sum = 0.0;

    for shape in shapes {
        sum += shape.area();
    }

    sum
}

trait Shape {
    fn area(&self) -> f64;
    
    fn is_my_area_gt<S: Shape>(&self, other: S) -> bool
        where Self: Sized;
}

struct Rectangle {
    length: f64,
    height: f64,
}

struct Square {
    length: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }

    fn is_my_area_gt<S: Shape>(&self, other: S) -> bool {
        self.area() > other.area()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }

    fn is_my_area_gt<S: Shape>(&self, other: S) -> bool {
        self.area() > other.area()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        f64::consts::PI * self.radius * self.radius
    }

    fn is_my_area_gt<S: Shape>(&self, other: S) -> bool {
        self.area() > other.area()
    }
}
