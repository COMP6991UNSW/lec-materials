//! # This is my library!
//! 
//! ...

// #![warn(missing_docs)]


pub enum Option<T> {
    Some(T),
    None
}

impl<T> Option<T> {

    /// Returns `true` if the option is a [`Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::option::Option;
    /// # 
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.is_some(), false);
    /// ```
    pub const fn is_some(&self) -> bool {
        
        matches!(*self, Self::Some(_))
    }
}



use hello::Foo;

pub mod a;

pub mod hello {
    pub struct Foo {
        /// This is the `x` field!
        pub x: i32,
        pub(crate) y: String,
    }

    impl Foo {
        pub fn new() -> Self {
            Foo {
                x: 42,
                y: String::from("foo"),
            }
        }

        pub fn y_len(&self) -> usize {
            self.y.len()
        }
    }


}

fn private() {
    let my_foo = Foo::new();
    let x = my_foo.x;
    let y = my_foo.y;

    println!("secret shhhhh");
}