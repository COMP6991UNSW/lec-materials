use std::fs::File;

fn main() {
    let file = match File::open("foo.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("ERROR: {err:?}");
            return;
        }
    };

    let file = File::open("foo.txt");
}
