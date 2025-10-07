
use zacs_module::{sub, mod3};

fn do_some_operations(left: u64, right: u64) {
    let added = add(left, right);
    let subed = sub(added, 10);

}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod point;
pub use point::Point;
pub use bmp::Image;

pub mod my_folder;

mod zacs_module {
    pub fn sub(left: u64, right: u64) -> u64 {
        mod3::my_fn();
        left - right
    }

    pub mod mod1 {}
    mod mod2 {}
    pub mod mod3 {
        pub fn my_fn() {
            super::super::add(2, 2);
            println!("hello from my_fn");
        }
    }
}





























#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
