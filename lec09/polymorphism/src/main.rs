#![allow(unused)]

use std::{collections::LinkedList, error::Error, f64::consts::PI, fmt::Display, fs::File};

struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

impl Default for Student {
    fn default() -> Self {
        Self { name: String::from("Simon"), zid: 5678901, wam: None }
    }
}

impl Clone for Student {
    fn clone(&self) -> Self {
        return Student {
            name: self.name.clone(),
            zid: self.zid,
            wam: self.wam,
        };
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Marker trait: has no required methods
// Requirement: all fields must be Copy (a bit magic)
// impl Copy for Student {}

fn main() -> Result<(), Box<dyn Error>> {
    // let square: Box<Square> = Box::new(Square { side_length: 5.0 });
    // let rectangle: Box<Rectangle> = Box::new(Rectangle { width: 5.0, height: 2.0 });

    // let shape1: Box<dyn Shape> = square;
    // let shape2: Box<dyn Shape> = rectangle;

    // let shape_vec: Vec<Box<dyn Shape>> = vec![shape1, shape2];

    // dbg!(sum_area(shape_vec));

    dbg!(sum_area(vec![
        Box::new(Square { side_length: 5.0 }) as Box<dyn Shape>,
        Box::new(Rectangle { width: 5.0, height: 2.0 }),
    ]));



    if true { return Ok(()) }


    // let file = File::open("hello.txt")?;
    let student = Student::default();
    let student: Student = Default::default();

    [1, 2, 3].into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter()
        .into_iter();

    // for i in [1, 2, 3] {
    //     // code
    // }

    // let mut iter = [1, 2, 3].into_iter();
    // loop {
    //     let i = match iter.next() {
    //         Some(next) => next,
    //         None => break,
    //     };

    //     // code
    // }


    // let s = student.to_string();

    dbg!(smallest(vec![5, 10]).unwrap());
    dbg!(smallest(vec![12.34, 3.14]).unwrap());
    dbg!(smallest(&vec!['z', 'a']).unwrap());
    dbg!(smallest_char('z', 'a'));
    dbg!(smallest(['z', 'a']).unwrap());
    dbg!(smallest(&['z', 'a']).unwrap());
    dbg!(smallest(LinkedList::from([1, 2, 3])).unwrap());
    dbg!(smallest('f'..='z').unwrap());
    
    dbg!('z'.smallest('a'));
    dbg!('a'.smallest('z'));

    // Turbofish:  ::<>

    let x: i32 = 42;
    let x = smallest_original::<i32>(42, 10);
    let x = smallest::<i32>(vec![1, 2, 3]);

    let thing = [1, 2, 3].into_iter()
        .map(|x| x * 2)
        .filter(|x| *x > 3)
        .collect::<Vec<_>>();

    // let opt: Option<i32> = Some(42);
    // let opt: Option<i32> = None;

    // fn Option(type: T) -> Option<T>

    // pub enum OptionI32 {
    //     None,
    //     Some(i32),
    // }
    //
    // let file = File::open("hello.txt")?;

    Ok(())
}

trait Smallest {
    fn smallest(self, other: Self) -> Self;
}

impl<T> Smallest for T
where 
    T: PartialOrd,
{
    fn smallest(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}


// bounded parametric polymorphism
/// If multiple elements are equal to smallest,
/// this fn will return the first one!
fn smallest<T>(xs: impl IntoIterator<Item = T>) -> Option<T>
where
    T: PartialOrd,
{
    let mut iter = xs.into_iter();
    let mut smallest = iter.next()?;

    for item in iter {
        if item < smallest {
            smallest = item;
        }
    }

    Some(smallest)
}

// useful for returning things like iterators!
// RPIT: Return Position Impl Trait
// (as opposed to) APIT: Argument Position Impl Trait
fn existential() -> impl Iterator<Item = (i32, i32)> {
    [1, 2, 3].into_iter()
        .map(|x| x * 2)
        .filter(|x| *x > 3)
        .zip(0..)
        .peekable()
}

fn smallest_original<T>(x: T, y: T) -> T
where 
    T: PartialOrd,
{
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_i32(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_f64(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_char(x: char, y: char) -> char {
    if x < y {
        x
    } else {
        y
    }
}







fn sum_area(shapes: impl IntoIterator<Item = Box<dyn Shape>>) -> f64 {
    let mut sum = 0.0;
    
    for shape in shapes {
        sum += shape.area();
    }

    sum
}

fn sum_area_impl<S: Shape>(shapes: Vec<S>) -> f64 {
    let mut sum = 0.0;
    
    for shape in shapes {
        sum += shape.area();
        shape.is_larger_than(todo!());
    }

    sum
}

enum ShapeEnum {
    Square(Square),
    Rectangle(Rectangle),
    Circle(Circle),
}

impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Square(square) => square.area(),
            ShapeEnum::Rectangle(rectangle) => rectangle.area(),
            ShapeEnum::Circle(circle) => circle.area(),
        }
    }
}

trait Shape {
    fn area(&self) -> f64;

    fn is_larger_than<S>(&self, other: S) -> bool
    where
        Self: Sized;
}

struct Square {
    side_length: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }

    fn is_larger_than<S>(&self, other: S) -> bool
    where
        Self: Sized,
    {
        todo!()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn is_larger_than<S>(&self, other: S) -> bool
        where
            Self: Sized {
        todo!()
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn is_larger_than<S>(&self, other: S) -> bool
        where
            Self: Sized {
        todo!()
    }
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x * 2)
}
