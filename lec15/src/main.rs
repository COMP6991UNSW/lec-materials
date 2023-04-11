fn foo(x_ptr: &mut Option<*const i32>) {
    let x = 42;
    *x_ptr = Some(&x as *const i32);
}

fn main() {
    let mut x_ptr: Option<*const i32> = None;

    foo(&mut x_ptr);

    unsafe {
        println!("x is {}", *x_ptr.unwrap());
    }
}
