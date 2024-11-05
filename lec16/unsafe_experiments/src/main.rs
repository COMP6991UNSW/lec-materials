fn foo() -> &'static i32 {
    let x = 42;
    unsafe { &* (&x as *const _) }
}

fn main() {
    let ptr = foo();

    println!("{ptr}");
}
