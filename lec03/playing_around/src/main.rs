struct Foo {
    x: i32,
    y: char,
    z: [i32; 5],
}

// type FooTuple = (i32, char, f32);

fn main() {
    let number = 42;
    let number2 = number;
    println!("{number}");

    // let string = String::from("hello");
    // let string2 = string;
    // println!("{string}");

    let y = (42,);

    let mut x: (i32, char, f32) = (42, 'x', 3.14);
    let (mut n, c, mut f) = x;
    // (n, c, f)   =   (42, 'x', 3.14)
    println!("{n} {c} {f}");
    let (n, ..) = x;
    let (_, c, _) = x;
    let (.., f) = x;

    let n = x.0;
    let c = x.1;
    let f = x.2;

    x.0 = 123;

    println!("{x:?}");
    println!("{:?}", x);

    let y: [i32; 5] = [1, 2, 3, 4, 5];
    let x: bool = false;
    let z: () = ();

    println!("{}", if x { 42 } else { 123 });
    let z = if x {
        42
    } else {
        123
    };

    let x = loop {
        // ...
        break;
    };

    //while some_fn_returns_true() {

    //}


    // let foo = Foo {
    //     x: 5,
    //     y: 'y',
    // };

    // match x {
    //     0 => {}
    //     1 => {}
    //     2 => {}
    //     _ => {}
    // }

    println!("Hello, world!");
}
