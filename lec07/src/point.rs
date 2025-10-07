//! I am writing some documentation here!

/// # The `Point` struct
///
/// `x` and `y` both exist only in the range -100 to +100
///
/// Please enjoy using this `struct`!
/// I think you'll like the [Point::add] function!
#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// This is the new function!
    pub fn new(x: i32, y: i32) -> Option<Self> {
        //! Hello, enjoy the function!
        if (-100..=100).contains(&x) && (-100..=100).contains(&y) {
            Some(Self { x, y})
        } else {
            None
        }
    }

    /// This is the add function!
    ///
    /// ```
    /// # use my_lib::point::Point;
    /// #
    /// let point1 = Point::new(2, 5).unwrap();
    /// let point2 = Point::new(3, 1).unwrap();
    /// 
    /// let added = point1.add(point2).unwrap();
    /// assert_eq!(added, Point::new(5, 6).unwrap());
    /// ```
    /// 
    pub fn add(self, rhs: Self) -> Option<Self> {
        let new_x = self.x + rhs.x;
        let new_y = self.y + rhs.y;

        Self::new(new_x, new_y)
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    // pub fn set_x(&mut self, new_x: i32) {
    //     self.x = new_x;
    // }

    // pub fn set_y(&mut self, new_y: i32) {
    //     self.y = new_y;
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn point_add_works1() {
        let point1 = Point::new(2, 5).unwrap();
        let point2 = Point::new(3, 1).unwrap();
        
        let added = point1.add(point2).unwrap();
        assert_eq!(added, Point::new(5, 6).unwrap());
    }

    #[test]
    fn point_add_works2() {
        let point1 = Point::new(2, 5).unwrap();
        let point2 = Point::new(3, 1).unwrap();
        
        let added = point1.add(point2).unwrap();
        assert_eq!(added, Point::new(5, 6).unwrap());
    }

    #[test]
    fn point_add_works3() {
        let point1 = Point::new(2, 5).unwrap();
        let point2 = Point::new(3, 1).unwrap();
        
        let added = point1.add(point2).unwrap();
        assert_eq!(added, Point::new(5, 6).unwrap());
    }
}
