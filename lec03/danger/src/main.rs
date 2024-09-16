#[derive(Copy, Clone)]
struct Foo {
    x: i32,
    y: usize,
    z: Option<f64>,
}

fn main() {
    let x = 42;
    let y = x;

    println!("{x}");

    let x = Box::new(42);
    let y = x.clone();

    println!("{x}");

    let foo1 = Foo { x: 42, y: 123, z: None };
    let foo2 = foo1;
    
    println!("{}", foo1.x);

    let ptr = give_me_int();
    print_int(ptr.clone());

    println!("{ptr}");
}

fn print_int(ptr: Box<i32>) {
    println!("{ptr}");
    // drop(ptr);
}

fn give_me_int() -> Box<i32> {
    Box::new(42)
}
