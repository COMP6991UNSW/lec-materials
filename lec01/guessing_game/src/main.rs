use std::io::stdin;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = loop {
            let mut guess_line = String::new();

            match stdin().read_line(&mut guess_line) {
                Ok(_) => {}
                Err(_) => {
                    println!("Bad input!");
                    continue;
                }
            }

            let guess_number: i32 = match guess_line.trim().parse() {
                Ok(parsed) => parsed,
                Err(_) => {
                    println!("It needs to be a number!!!");
                    continue;
                }
            };

            break guess_number;
        };

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
