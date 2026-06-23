#![warn(missing_docs)]
//! Welcome to my `lec07` crate!
//!
//! Please enjoy your stay!

mod a {
    mod b {
        mod c {
            mod d {
                pub(super) fn foo() {}
            }
        }
    }
}


/// Some documentation for mod students
pub mod students; // students.rs


pub enum ZacsEnum<T> {
    Some(T),
    None,
}

/// # Add function
///
/// This function adds the **two parameters**
/// `x` and `y` together!
///
/// `x`: any i32 that you please!
///
/// `y`: also any i32 that you please!
///
/// ### Example
///
/// ```
/// # use lec07::add;
/// let answer = add(15, 27);
/// assert_eq!(answer, 42);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    //! Some more documentation
    //!
    //! But inside the function this time!!!

    students::some_student_fn();
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add1() {
        let result = add(3, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_add2() {
        let result = add(3, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_add3() {
        let result = add(3, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_add4() {
        let result = add(3, 4);
        assert_eq!(result, 7);
    }
}
