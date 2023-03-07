//! # Lec07 Crate
//! 
//! - This
//! - is
//! - my
//! - list
//! 
//! This is a crate that is used to demonstrate
//! modules, testing, and visibility.


#![allow(unused)]
// #![warn(missing_docs)]


pub mod one_file_module;
pub mod folder_mod_1;
pub use folder_mod_1::function_from_a;

pub mod folder_mod_2;


pub mod inline_module {
    pub fn foo() {
        println!("Foo!");
    }
}


pub use folder_mod_1::a::Foo;


/// # Some notice
/// 
/// This function is useful
/// if used alongside the
/// [`Foo`] type.
/// 
/// # Warning
/// 
/// This function will panic if
/// `x` or `y` are less than 0.
pub fn add_two(x: i32, y: i32) -> i32 {
    if x < 0 || y < 0 {
        panic!("x and y must not be negative!!");
    }

    x + y
}

/// Some stuff about the function
fn fn_one() {
    fn_two();
}

fn fn_two() {
    fn_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = add_two(2, 3);
        println!("THE RESULT IS {result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(add_two(5, 10), 15);
    }

    #[test]
    fn test_3() {
        assert_eq!(add_two(40, 2), 42);
    }

    #[test]
    #[should_panic]
    fn test_4() {
        add_two(-1, 50);
    }
}