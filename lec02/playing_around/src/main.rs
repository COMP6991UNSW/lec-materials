const NUM_ROWS: i32 = 7 + 2;

#[derive(Debug, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    description: String,
}

enum Color {
    Red,
    Green,
    Blue,
    Other(Pixel),
}

enum HandSign {
    ThumbsUp,
    ThumbsDown,
}

enum PossibleErrors {
    NothingToRead,
    RanOutOfSpace,
    NotImplemented,
    OtherError(String),
}

fn read_line() -> Result<String, PossibleErrors> {
    Err(PossibleErrors::NotImplemented)
}

fn generate_pixel(c: Color) -> Result<Color, String> {
    let pixel = match c {
        Color::Red => Result::Ok(Pixel { r: 255, g: 0, b: 0 }),
        Color::Green => Result::Ok(Pixel { r: 0, g: 255, b: 0 }),
        Color::Blue => Result::Ok(Pixel { r: 0, g: 0, b: 255 }),
        Color::Other(o) => Result::Err("Need to send in Red, Green or Blue".to_string()),
    };

    return pixel;
}

fn main() {
    let x: [i32; 3] = [1, 2, 3];

    let mut t: (i32, String) = (3, "MyString".to_string());

    t.0 = t.0 + 1;

    println!("{}", x[0]);
    println!("{}", t.1);

    // Expressions
    (3 + 4);

    // Statement
    let x = 3;

    let greater_than_zero = if x > 0 {
        if x == 0 {
            "Equal".to_string()
        } else {
            "GreaterThan".to_string()
        }
    } else {
        "Less Than".to_string()
    };

    let mut a = 1;
    let mut b = 1;

    let my_fib_value = loop {
        let tmp = a + b;
        a = b;
        b = tmp;

        if b > 100 {
            break a + b;
        }
    };

    for i in 0..10 {}
}

fn power(base: i32, exponent: u32) -> Option<i32> {
    if base == 0 && exponent == 0 {
        None
    } else {
        let mut x = 1;
        for _ in 0..exponent {
            x = x * base;
        }
        Some(x)
    }
}

fn print(s: String) {
    println!("{s}");
}

fn print(s: String, i: i32) {
    println!("{s}");
}

fn ownership() {
    let x = Pixel { r: 3, g: 5, b: 7 };

    println!("{:?}", x);

    let mut y = x.clone();

    y.r = 72;

    // String Example

    let s = "My String".to_string();

    print_string(s);

    println!("{s}");
}
