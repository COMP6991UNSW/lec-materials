#![allow(unused)]

// Product type
struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

// Sum type
enum CarBrand {
    Toyota(Option<i32>),
    Nissan(String),
    Subaru,
    // ...
}

#[derive(Clone, Copy)]
struct Foo {
    x: i32,
    y: f64,
}

// fn create_student(...) -> Result<Student {
//     assert!(0 <= wam && wam <= 100)
// }

// f32 and f64
// float and double

const MY_SPECIAL_NUMBER: i32 = 42;

fn main() {
    let my_option: Option<i32> = Some(42);
    let my_option: Option<i32> = None;

    let x = 42;
    let y = 123;
    
    let (x, y) = (y, x);

    // match 42 {
    //     0 => todo!(),
    //     1 => todo!(),
    //     2 => todo!(),
    //     3 => todo!(),
    //     _ => todo!(),
    // }

    let expression_value = match my_option {
        Some(value) => {
            println!("The value is {value}");

            value
        }
        Some(42) => {
            println!("42!");

            42
        }
        hello => {
            println!("Value = {hello:?}");

            999
        }
    };

    if my_option.is_some() {
        let value = my_option.unwrap();
    }

    println!("{expression_value}");

    if let Some(value) = my_option {
        println!("The value exists, and it is {value}");
    }


    let my_result: Result<i32, String> = Err(String::from("oh no!"));
    match my_result {
        Ok(number) => {
            println!("Ok! The number is {number}");
        }
        Err(message) => {
            println!("Err! The message is {message}");
        }
    }

    if true { return }


    let brand: CarBrand = CarBrand::Toyota(None);
    let brand: CarBrand = CarBrand::Nissan(String::from("hello"));
    let brand: CarBrand = CarBrand::Subaru;

    match brand {
        CarBrand::Toyota(Some(number)) => todo!(),
        CarBrand::Toyota(None) => todo!(),
        CarBrand::Nissan(string) => todo!(),
        CarBrand::Subaru => todo!(),
    }

    let mut my_array = [
        1,
        2,
        3,
        4,
        5,
    ];

    for elem in my_array {
        println!("{elem}");
    }

    let x = loop {
        // ...
        break 42;
    };

    if true { return; }

    my_array[0] = 42;

    let my_vec = vec![1, 2, 3];

    let my_tuple: (bool, i32, char) = (true, 42, 'z');
    // let student_tuple: Student = todo!();

    let my_tuple: (i32,) = (5 + 2,);
    let unit_tuple: () = ();

    let x = 10;

    let my_variable = if x == 42 {
        let y = 5;
        
        y * 2
    } else {
        50
    };

    let my_variable = if x == 42 { 10 } else { 50 };

    // let my_2nd_variable = if x == 42 {
    //     10
    // };



    // i8, i16, i32, i64, i128
    // u8, u16, u32, u64, u128
    // char, short, int, long, long long

    // 32-bit 64-bit
    // isize, usize
    //
    // char bool
    
    let a: bool = false;
    let c: char = 'a';

    let x: i32 = 42;
    let y: u8 = x.try_into()
        .expect("Number was too big");
    println!("{y}");

    let mut student = Student {
        name: String::from("Student McStudentFace"),
        zid: 5555555,
        wam: Some(100.0),
    };

    student.wam = Some(50.0);

    // start of scope a
    let x = 42;
    let mut x = 500;

    let a = {
        let mut a = 42;
        a *= 2;
        a += 50;
        a /= 3;
        a -= 100;
        
        a
    };

    let x0 = 42;
    {
        // start of scope b
        let x1 = 123;

        println!("The value is: {x1:?}");
        // end of scope b
    }
    println!("The value is {x0}");

    for x in 0..10 {

    }

    println!("{x}");
    // end of scope a
    

    let my_string = String::from("hello");
    let my_string = takes_string(my_string);

    println!("{my_string}");
}

fn add_5(x: i32) -> i32 {
    let value = x + 5;
    value
}

fn takes_string(x: String) -> String {
    println!("{x}");

    x
}
