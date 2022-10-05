//! # A folder module!
//! 
//! Folder modules must contain a file
//! named `mod.rs`, that acts as the
//! entry-point for the overall module.
//! 
//! It is very idiomatic for `mod.rs`
//! files to simply re-export submodule
//! contents.
//! 
//! As such, we will do that in this module!
//! 
//! We can also put doctests here too!
//! 
//! # Example
//! 
//! ```
//! # use lec08::folder_module;
//! assert_eq!(folder_module::function_from_a(), 42);
//! assert_eq!(folder_module::function_from_b(), 42);
//! assert_eq!(folder_module::function_from_c(), 42);
//! ```

pub mod a;
pub mod b;

/// You can also document modules here!
pub mod c;

pub use a::function_from_a;
pub use b::function_from_b;
pub use c::function_from_c;
