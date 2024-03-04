//! # My epic library!
//! 
//! This is so cool!

/// My favourite number function documentation!
/// 
/// # Example
/// 
/// this is my example
/// 
/// ```
/// // let f = my_favourite_number();
/// ```
/// 
/// # List of favourite numbers
/// 
/// 1. 42
/// 2. 100
/// 3. 123
pub fn my_favourite_number() -> i32 {
    42
}

/// Doc before
pub fn foo() {
    //! Doc after This is documentation for foo
    //! 
    //! etc. etc.
}

/// # The Add 5 function
/// 
/// In all it's glory!!!
/// 
/// # Example
/// 
/// ```
/// # use mylib::add_5;
/// assert_eq!(add_5(0), 5);
/// assert_eq!(add_5(5), 10);
/// ```
pub fn add_5(x: i32) -> i32 {
    x + 5
}

#[cfg(test)]
mod tests {
    use crate::add_5;

    #[test]
    fn it_works1() {
        assert_eq!(5, add_5(0));
    }

    #[test]
    fn it_works2() {
        assert_eq!(6, add_5(1));
    }

    #[test]
    fn it_works3() {
        assert_eq!(0, add_5(-5));
    }
}