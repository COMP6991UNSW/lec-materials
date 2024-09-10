use core::f64;

// product type
struct Student {
    name: String,
    age: u32,
    wam: Option<f64>,
}

enum CoinType {
    Five,
    Ten,
    Twenty,
    Fifty,
    Dollar,
    TwoDollar,
}

enum NoteType {
    Five,
    Ten,
    Twenty,
    Fifty,
    Hundred,
}

// sum type
enum Money {
    Coin(CoinType),
    Note(NoteType),
}

const INVALID_WAM: f64 = -1.0;

fn main() {
    // standard for loop (post lecture)
    for i in 0..10 {
        println!("{i}");
    }

    // standard while loop (post lecture)
    let mut x = 0;
    while x <= 10 {
        println!("{x}");
        x += 1;
    }

    // using loop as an expression (post lecture)
    let x = loop {
        if true {
            break 42;
        }

        break 123;
    };


    let student = Student {
        name: String::from("Jeff"),
        age: 123,
        wam: None,
    };

    match student.wam {
        Some(wam) => {
            println!("{}", wam);
        }
        None => {
            println!("student does not have wam");
        }
    }

    let money = Money::Note(NoteType::Twenty);

    // Match on the money enum (post lecture)
    match money {
        Money::Coin(coin_type) => {
            match coin_type {
                CoinType::Five => {}
                CoinType::Ten => {}
                CoinType::Twenty => {}
                CoinType::Fifty => {}
                CoinType::Dollar => {}
                CoinType::TwoDollar => {}
            }
        }
        Money::Note(note_type) => {
            // ...
        }
    }
}
