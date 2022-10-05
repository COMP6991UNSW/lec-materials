//! # A single file module.
//! 
//! This is a module defined in a single file.
//! 
//! This single file comprises the module
//! in its totality.
//! 
//! Please enjoy this single file module at your leisure!



/// # A private function
/// 
/// This function cannot be called from outside
/// of this module, or any supermodules.
fn private_function() -> i32 {
    42
}

/// # Another private function
/// 
/// Like [`private_function`], this function
/// cannot be called from outside
/// of this module, or any submodules.
/// 
/// In other words, `pub(self)` is the same
/// behaviour as not specifying a visibility
/// modifier, which defaults to private.
pub(self) fn also_private_function() -> i32 {
    42
}

/// # A somewhat private function
/// 
/// `pub(in <path>)` allows you to specify a
/// visibility within any ancestor module.
/// As such, `semi_private_function` is visible
/// within the outer [`one_file_module`].
/// 
/// The same could be achieved
/// (in this particular case)
/// with `pub(in super)`
/// or simply `pub(super)`.
pub(in crate::one_file_module) fn semi_private_function() -> i32 {
    42
}

/// # A somewhat private function
/// 
/// The same as [`semi_private_function`]
pub(super) fn also_semi_private_function() -> i32 {
    42
}

/// # A crate-visible function
/// 
/// This function is visible from anywhere
/// within the current crate, but not ouside
/// of it.
/// 
/// This is also the same as
/// `pub in(crate)`.
pub(crate) fn crate_visible_function() -> i32 {
    42
}

/// # A truly public function!
/// 
/// This function can be called from
/// anywhere, within this crate or not!
/// 
/// Great news for API consumers as they
/// can use this! You'll definitely want
/// some good documentation to go along
/// with it, as well as a documentation
/// test!
/// 
/// ```
/// # use lec08::one_file_module::public_function;
/// let my_number = public_function();
/// assert_eq!(my_number, 42);
/// ```
pub fn public_function() -> i32 {
    42
}


