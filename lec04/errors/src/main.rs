use std::fs::File;
use std::io::Read;

fn main() {
    let mut file: File = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open the file: {err:?}");
            return;
        }
    };

    let mut line = String::new();
    match file.read_to_string(&mut line) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to read from the file: {err:?}");
            return;
            // ...
        }
    }

    println!("Line: {line}");
}
