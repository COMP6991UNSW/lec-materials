#[derive(Copy, Clone)]
struct Student {
    // name: String,
    zid: u32,
    wam: Option<f64>,
}

fn main() {
    let x = 42;
    let y = x;
    println!("{x}");
    println!("{y}");

    let x = String::from("cat");
    let mut x = x;
    x.make_ascii_uppercase();
    let x = print_string(x);

    println!("{x}");
}

fn print_string(mut s: String) -> String {
    s.make_ascii_uppercase();
    println!("{s}");
    s
}
