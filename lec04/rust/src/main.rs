// Result definition:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

enum GetInputError {
    IoError(std::io::Error),
    InputEmpty,
    InputWasNotNumber {
        input: String,
    },
    // more errors here...?
}

impl From<std::io::Error> for GetInputError {
    fn from(error: std::io::Error) -> Self {
        GetInputError::IoError(error)
    }
}

fn get_input() -> Result<i32, GetInputError> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)?;

    if input.is_empty() {
        return Err(GetInputError::InputEmpty);
    }

    let num = input.trim().parse()
        .map_err(|_| GetInputError::InputWasNotNumber { input })?;

    Ok(num)
}

// {
//     THIS:
//     let x = match foo {
//         Ok(val)  => val,
//         Err(err) => return Err(err.into()),
//     };
//
//     IS EQUIVALENT TO:
//     let x = foo?;
// }

fn helper1() -> i32 {
    loop {
        match get_input() {
            Ok(value) => {
                return value;
            }
            Err(err) => {
                match err {
                    GetInputError::IoError(_) | GetInputError::InputEmpty => {
                        panic!("Could not read input");
                    }
                    GetInputError::InputWasNotNumber { input } => {
                        println!("You wrote: {input}\nThat was not a number!");
                    }
                }
            }
        }
    }
}

fn main() {
    let number = helper1();
    println!("Number times 2 is {}", number * 2);


    // These ideas don't clash in the language!!
    // e.g.
    //
    // [1, 2, 3, 4, 5].into_iter()
    //     .map(|x| Err(42))
}
