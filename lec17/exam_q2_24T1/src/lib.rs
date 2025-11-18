#![allow(unused)]

use require_lifetimes::require_lifetimes;

/// This function prints the given input.
/// You will need to annotate its lifetimes.
/// (2 marks)
#[require_lifetimes]
pub fn print(a: &i32) {
    println!("{a}");
}

/// This function returns the first parameter it is given.
/// You will need to annotate its lifetimes.
/// (3 marks)
#[require_lifetimes]
pub fn first(a: &i32, b: &i32) -> &i32 {
    a
}

/// A struct to hold the data of a string being split.
/// You will need to annotate its lifetimes.
/// (2 marks)
pub struct StringSplitter {
    pub text: &str,
    pub pattern: &str,
}

/// This function creates a string splitter with given data.
/// You will need to annotate its lifetimes.
/// (3 marks)
#[require_lifetimes]
pub fn split(text: &str, pattern: &str) -> StringSplitter {
    StringSplitter {
        text,
        pattern,
    }
}
