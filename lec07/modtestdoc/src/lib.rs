//! 

use my_module::{Foo, give_int};

mod my_module;
mod utils;



/// # Add function
/// 
/// This function adds together two numbers,
/// `left` and `right`.
/// 
/// # Return value
/// 
/// This function returns the sum of the input numbers.
/// 
/// # Example
/// 
/// ```
/// # use modtestdoc::add;
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn say_number() {
    let foo = Foo::new(42, 123);
    let x  = foo.x();

    let number = give_int();
    println!("The number is {number}");

    let foo = utils::a::foo();

    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works3() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
