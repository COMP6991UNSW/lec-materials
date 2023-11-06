use std::ptr::NonNull;

/// # Safety
///
/// In order to call `foo` safely, you need to make sure:
///
/// * `x` is a well-aligned valid pointer to an i32 that can be read from
/// * `y` is a well-aligned valid pointer to an i32 that can be read from
///
unsafe fn foo(x: *mut i32, y: *mut i32) -> i32 {
    unsafe { *x + *y }
}

fn main() {
    let a = 2;
    let b = 3;

    // /!\ SAFETY:
    //      Well, a is coming from a valid stack i32 pointer,
    //      and totally so is b im super sure!
    unsafe {
        let x = foo(&a as *const i32 as *mut i32, (&b as *const i32 as *mut i32).wrapping_add(100));
        println!("{x}");
    }

    println!("{a}");
}
