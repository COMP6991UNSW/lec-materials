use unsafe_review::Lazy;

fn main() {
    let lazy = Lazy::new(|| {
        println!("initializing...");
        42
    });

    // run this twice
    for _ in 0..2 {
        println!("reading from lazy...");
        let value = *lazy;
        println!("got {value}");
    }
}
