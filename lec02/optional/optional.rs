// build with: rustc optional.rs


fn create(b: bool) -> Option<&'static str> {
    if b {
        Some("Hello There")
    } else {
        None
    }
}

fn main() {
    // method 1
    let create_true = create(true);
    if create_true.is_some() {
        println!("create(true) returned {}", create_true.unwrap());
    }

    // method 2
    println!(
        "create(false) returned {}",
        create(false).unwrap_or("<empty>")
    );








    // ...
    // ...
    // ...













    let x: Option<i32> = None;
    // let x = Some(123);

    // let x: Option<Option<Option<i32>>> = Some(Some(Some(42)));


    // method 3
    let create_true: Option<&str> = create(true);
    match create_true {
        Some("")      => println!("create(true) returned empty string"),
        Some(string)  => println!("create(true) returned {string}"),
        Some("hello") => println!("create(true) returned hello"),
        None          => println!("create(false) returned <empty>"),
    }


    let bools: (bool, bool) = (true, false);
    match bools {
        (left, right) => {}
        // (false, true)  => {}
        // (true, false)  => {}
        // (true, true)   => {}
    }
}


// enum Option<T> {
//     None,
//     Some(T),
// }
// use Option::None;
// use Option::Some;
// 
