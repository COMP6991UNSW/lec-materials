use std::{fs::File, io::Read};

const PI: f64 = 3.14;

// enum Option<T> {
//     Some(T),
//     None,
// }
// 
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn unwrap<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(good_value) => Some(good_value),
        Err(_) => None,
    }
}


// product type: Student = (u32 * String * Option<f64>)
// sum type: WebBrowserEvent = (A + B + C + D)

struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

struct Key;

enum WebBrowserEvent {
    PageLoad(String),
    PageQuit,
    Click { x: u32, y: u32 },
    KeyPressDown(Key),
    KeyPressUp(Key),
}

fn log2(number: u32) -> Option<u32> {
    let result = if number == 0 {
        None
    } else {
        Some(number.ilog2())
    };

    println!("Function is done!");

    result
}

fn takes_a_file(file: File) -> File {
    // file.read(..);

    file
}

fn main() {
    // let (mut) <name>(: <type>_) = <value>;

    let mut my_student = Student {
        zid: 5555555,
        name: String::from("Daniel"),
        wam: Some(90.0),
    };

    my_student.wam = Some(95.0);

    let x: () = ();

    let mut x = 42;
    x = 45;

    let my_tuple: (i32, &str, bool, f64) = (42, "hello", false, 3.14);

    let i = 2;

    if i != 0 {
        
    }


    let (favourite_number, greeting, is_cool, pi) = my_tuple;
    println!("{favourite_number}, {greeting}, {is_cool}, {pi}");

    let greeting = if is_cool { "nice one dude!" } else { "not cool :(" };

    let hello_and_goodbye = {
        let name = "Zac";
        (format!("Nice to meet you {name}"), format!("Goodbye {name}"))
    };

    let (hello, goodbye) = hello_and_goodbye;

    let salary =
        if is_cool {
            println!("You are so cool!");
            100000
        } else {
            println!("You are not cool!");
            3
        };

    println!("Your salary is {salary}");


    let foo;
    if true {
        foo = 42;
    }
    // println!("{foo}");

    let my_array: [i32; 8] = [3, 1, 4, 1, 5, 9, 2, 6];
    let x = my_array[i];

    let event = WebBrowserEvent::Click { x: 50, y: 100 };

    const X: usize = 0;
    let y: [i32; X] = [];

    match event {
        WebBrowserEvent::Click { x, y } => {
            println!("They clicked at ({x}, {y})");
        }
        _ => {
            println!();
        }
    }

    let my_nested_tuple = ((1, true), 'c');

    let my_file: File = File::open("hello.txt").unwrap();
    let my_file = takes_a_file(my_file);



    let my_malloced_int: Box<i32> = Box::new(42);

    // Ancient haskell mantra:
    // Make illegal states unrepresentable!!!

    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    // f32, f64,

    // bool, char

    for elem in my_array {

    }
}
