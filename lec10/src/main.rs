use std::f64::consts::PI;

fn main() {
    let shape = heres_a_shape();
    dbg!(shape.area());

    dbg!(sum_areas(vec![
        &Square { side_length: 3.0 }, // 9
        &Square { side_length: 2.0 }, // 4
        &Square { side_length: 5.0 }, // 25
        &Circle { radius: 5.0 }, // 25
        &Rectangle { height: 5.0, width: 3.0 }, // 25
    ]));

    // let array = [1, 2, 3];
    // let (x, y, z) = array.into();

    // let mut v = vec![1, 2, 3];
    // let mut i = my_vec_iter(&v);
    // for item in i {
    //     println!("{item}");
    // }

    // v.into_iter()  -> impl Iterator<Item = T>
    // v.iter()       -> impl Iterator<Item = &T>
    // v.iter_mut()   -> impl Iterator<Item = &mut T>

    // let i = (&mut v).into_iter();
    // for item in i {

    // }
}

fn heres_a_shape() -> impl Shape {
    Rectangle {
        height: 3.0,
        width:  4.0,
    }
}

fn sum_areas(vec: Vec<&dyn Shape>) -> f64 {
    let mut sum = 0.0;

    for shape in vec {
        sum += shape.area();
    }

    sum
}

trait Shape {
    fn area(&self) -> f64;

    fn smaller_than<S: Shape>(&self, other: &S) -> bool
        where Self: Sized;
}

struct Square {
    side_length: f64,
}

struct Rectangle {
    height: f64,
    width: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }

    fn smaller_than<S: Shape>(&self, other: &S) -> bool {
        self.area() < other.area()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn smaller_than<S: Shape>(&self, other: &S) -> bool {
        self.area() < other.area()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn smaller_than<S: Shape>(&self, other: &S) -> bool {
        self.area() < other.area()
    }
}



fn my_vec_iter<'a, T>(vec: &'a Vec<T>) -> MyVecIter<'a, T> {
    MyVecIter {
        vec,
        position: 0,
    }
}

struct MyVecIter<'a, T> {
    vec: &'a Vec<T>,
    position: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.position)?;
        self.position += 1;

        Some(item)
    }
}
