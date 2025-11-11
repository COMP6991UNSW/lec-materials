use std::cell::Cell;

/// # SAFETY
///
/// DO NOT CALL THIS WITH `true`, IT WILL BE VERY BAD
unsafe fn foo(b: bool) {
    if b {
        // SAFETY: This code is unreachable,
        // and thus will never be called,
        // because we require that b is `false`,
        // and this is in a `if b` block.
        unsafe {
            let ptr: *const String = std::ptr::null();
            println!("Hello, world! My string is {}", *ptr);
        }
    }
}

static mut X: i32 = 0;

fn main() {
    let ptr: *const i32 = std::ptr::null();
    let ptr = ptr.wrapping_add(10);
    let x = *ptr;

    // SAFETY: We are not calling with `true`, as required by `foo`.
    unsafe { foo(false); }

    foo();
}

struct Foo {
    c: Cell<i32>,
}
// SAFETY: This is actually not safe whoops!
unsafe impl Sync for Foo {}
