use std::{fs::File, io::Read};

struct Foo {
    x: bool,
    y: bool,
    z: bool,
}

enum Bar {
    X(Box<Bar>, Box<Bar>),
    Y(f32),
    Z(i32, f32, String),
}

fn gives_bar() -> Bar {
    todo!()
}

fn main() {
    let file = File::open("hello.txt");
    match file {
        Ok(mut my_epic_file) => {
            let mut contents = String::new();
            my_epic_file.read_to_string(&mut contents);
            let first_line = contents.lines().next();

            if let Some(line) = first_line {
                println!("The line exists!");
            }

            let line = match first_line {
                Some(line) => {
                    line
                }
                None => {
                    "{{ no line exists }}"
                }
            };

            println!("{line}");
        }
        Err(error) => {
            println!("There was an error: {error:?}");
        }
    }
}
