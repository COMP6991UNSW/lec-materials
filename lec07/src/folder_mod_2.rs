//! # A folder module!
//! 
//! This style of folder-module has a file
//! with the same name as the folder containing
//! its submodules. This file acts as the
//! entry-point for the overall module.
//! 
//! It is very idiomatic for the entry-point
//! file to simply re-export submodule
//! contents.
//! 
//! As such, we will do that in this module!
//! 
//! We can also put doctests here too!
//! 
//! # Example
//! 
//! ```
//! # use lec07::folder_mod_2;
//! assert_eq!(folder_mod_2::function_from_a(), 42);
//! assert_eq!(folder_mod_2::function_from_b(), 42);
//! assert_eq!(folder_mod_2::function_from_c(), 42);
//! ```

pub mod a;
pub mod b;

/// You can also document modules here!
pub mod c;

pub use a::function_from_a;
pub use b::function_from_b;
pub use c::function_from_c;