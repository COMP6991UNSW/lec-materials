struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

enum Fruit {
    Apple,
    Pear,
    Peach,
    Mango,
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// enum Option<T> {
//     Some(T),
//     None
// }

enum ReadNumberError {
    CantReadInput,
    EnteredSomethingThatWasntANumber,
}

fn read_a_number() -> Result<i32, ReadNumberError> {
    let mut line = String::new();
    match std::io::stdin()
        .read_line(&mut line) {
            Ok(_) => {},
            Err(_) => return Err(ReadNumberError::CantReadInput),
    }

    let number = line.parse::<i32>()
        .map_err(|_| ReadNumberError::EnteredSomethingThatWasntANumber)?;

    Ok(number)
}

fn main() {
    let the_number = read_a_number();
    
    // DOESN'T COMPILE!!
    // println!("Your number times 2 is {}", the_number * 2);

    println!("Hello, world!");
}
