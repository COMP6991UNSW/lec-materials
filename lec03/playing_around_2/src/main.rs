fn main() {
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 42;
    let x = 25 + 17;
    let x = 21 * 2;

    give_random_number();

    if x == give_random_number() {

    }

    let my_number = {
        let xyz = 123;
        println!("{xyz}");
        xyz * 5
    };

    println!("{my_number}");

    let input = loop {
        let user_input = "...";
        if user_input == "..." {
            break user_input;
        }
        println!("bad input for some reason");
    };

    let x = if true { 42 } else { 123 };
    // PHP ternary associativity!!!!

    println!("{}", if x == add_two_numbers(25, 17) {
        println!("this is gonna be good!");
        "hooray!"
    } else {
        "sad :("
    });
}

fn give_random_number() -> i32 {
    if false {
        return 5;
    }

    println!("thinking of a random number...");

    if true {
        4
    } else {
        3
    }
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn print_hello() {
    println!("hello!");
}
