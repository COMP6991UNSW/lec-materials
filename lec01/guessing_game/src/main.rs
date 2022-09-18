use std::io;
use std::cmp::Ordering;

use rand::Rng;

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let number = input.trim().parse::<i32>()
            .unwrap();

        match number.cmp(&secret_number) {
            Ordering::Less => {
                println!("Guessed too low!");
            }
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Greater => {
                println!("Guessed too high!");
            }
        }
    }
}
