pub mod b;
pub mod c;

/// # This is the `add_2` function.
/// 
/// It takes in two variables, `x` and `y`.
/// It will return their _sum_.
/// Go check out [our website](https://google.com)! 
/// 
/// You don't have to be **so** careful with this one!
pub fn add_2(x: i32, y: i32) -> i32 {
    x + y
}

pub fn foo() {
    b::foo();
}



#[cfg(test)]
mod tests {
    use super::add_2;

    #[test]
    fn my_test1() {
        assert_eq!(add_2(2, 3), 5);
    }

    #[test]
    fn my_test2() {
        assert_eq!(add_2(2, 3), 5);
    }

    #[test]
    fn my_test3() {
        assert_eq!(add_2(2, 3), 5);
    }
}