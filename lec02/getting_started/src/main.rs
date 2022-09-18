struct Student {
    zid: String,
    age: u32,
    wam: Option<f64>,
}

// product-types

enum DrivenWheels {
    FWD,
    FOURWD,
    AWD,
    RWD,
}

enum Automobile {
    Car { make: String, model: String, doors: u32, driven_wheels: DrivenWheels },
    Motorbike { make: String, model: String, horsepower: u32 },
    Train { capacity: u32 },
    Bus { capacity: u32 },
    Plane { engines: u32 },
}

// sum-types

type Student1 = (String, u32, Option<f64>);

fn foo1(automobile: Automobile) {
    match automobile {
        Automobile::Car { make, model, doors, driven_wheels } => {
            println!("It's a car! made by {make} -- it is the {model} with {doors} doors");
        }
        Automobile::Motorbike { make, model, horsepower } => {
            
        }
        Automobile::Train { capacity } => {
            
        }
        _ => {}
    }
}

fn takes_an_option(x: Result<i32, String>) {
    match x {
        Ok(calculation) => {
            println!("The calculation was {calculation}");
        }
        Err(err_message) => {
            eprintln!("There was an error: {err_message}");
        }
    }
}

fn log_2(number: f32) -> Option<f32> {
    let logarithm: f32 = if number == 0.0 {
        return None;
    } else {
        number.log2()
    };

    Some(logarithm)
}





#[derive(Copy, Clone)]
struct Foo {
    x: i32,
    y: u32,
    z: bool,
}


fn takes_a_string(s: &str) {
    println!("I got the string {s}");
}


fn ownership() {
    let s = String::from("Hello!");

    takes_a_string(&s);

    println!("I got {s} back");
    drop(s);
}










fn main() {
    ownership();
    // let x = [1, 2, 3, 4, 5];
    // for element in x {
    //     println!("{element}");
    // }
}
