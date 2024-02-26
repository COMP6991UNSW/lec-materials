#![allow(unused)]

mod pain;
mod shared;
mod exclusive;
mod dangling;

struct Student {
    zid: u32,
    name: String,
    wam: Option<f64>,
}

fn main() {
    let student = Student {
        zid: 5555555,
        name: String::from("hello"),
        wam: None,
    };

    // match:   &name
    //          &String::from("hello")
    //
    //          name = String::from("hello")

    match &student {
        Student { zid, name, wam } => {
            println!("{name}");
        }
    }

    match &student {
        &Student { zid, ref name, wam } => {
            println!("{name}");
        }
    }

    match student {
        Student { zid, ref name, wam } => {
            println!("{name}");
        }
    }

    println!("{}", student.name);
}

