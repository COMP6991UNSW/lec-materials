use std::io::stdin;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Failed to read the guess");

        let guess: i32 = guess.trim().parse().expect("Guess is not a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too low!");
            }
            Ordering::Greater => {
                println!("Too high!");
            }
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
