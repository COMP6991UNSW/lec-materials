static mut COUNTER: i32 = 0;

fn main() {
    let ptr: *mut i32 = std::ptr::null_mut();
    let val = unsafe { *ptr };

    // SAFETY: ...
    unsafe {
        COUNTER += 1;
        println!("{COUNTER}");
    }
        

    println!("Hello, world!");
}

// # Safety:
//
// * .
// * .
// * .
unsafe fn foo(ptr: *mut i32) {
    // SAFETY:
    // This is actually ok because:
    // * .
    // * .
    // * .
    // * .
    let vec: Vec<i32> = unsafe {
        Vec::from_raw_parts(ptr, 5, 20)
    };
}
