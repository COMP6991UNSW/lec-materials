// #![warn(missing_docs)]

//! Here is some more documentation!!
//!
//! Wow there's so much!
//!
//! I suggest you try out the [Point] struct -- it's so cool!

use std::{mem::swap, vec::Vec};




/// This struct holds a coordinate pair of `i32`s.
///
/// It's such a cool struct ur gonna love it!
///
/// # This is a header!
///
/// ```
/// // this is some code
/// let x = 42;
/// ```
///
/// Try out the [Point::new] function!
///
/// Here is some **bold text** and here's some *italics*.
#[derive(Debug, PartialEq)]
pub struct Point {
    /// This is the `x` field! So cool!
    pub x: i32,

    /// And the `y` field, awesome too
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }


    fn hello(&self) {
        println!("hello from {} {}", self.x, self.y);
    }

    /// This is the add function, bla bla bla
    ///
    /// ```
    /// # use my_library::point::Point;
    /// #
    /// let point1 = Point { x: 2, y: 3 };
    /// let point2 = Point { x: 3, y: 2 };
    ///
    /// let expected = Point { x: 5, y: 5 };
    /// let result = point1.add(point2);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn add(&self, other: Point) -> Point {
        let v: Vec<()> = Vec::new();
        let v: Vec<()> = std::vec::Vec::new();

        crate::add(Point { x: 4, y: 2 });

        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_1() {
        let point1 = Point { x: 2, y: 3 };
        let point2 = Point { x: 3, y: 2 };

        let expected = Point { x: 5, y: 5 };
        let result = point1.add(point2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_2() {

    }

    #[test]
    fn test_add_3() {

    }
}
