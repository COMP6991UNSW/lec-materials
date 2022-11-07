pub fn main() {
    let my_string: &'static str = "hello";

    std::thread::spawn(move || {
        println!("{my_string}");
    }).join().unwrap();
}
