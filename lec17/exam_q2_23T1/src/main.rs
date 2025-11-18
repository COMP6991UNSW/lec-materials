use exam_q2_lib::split_once;

fn main() {
    let string = String::from("hello world");
    let tup: Option<(&str, &str)>;

    {
        let split_on = String::from(" ");
        tup = split_once(string.as_str(), split_on.as_str());
    }

    println!("{tup:?}");
}
