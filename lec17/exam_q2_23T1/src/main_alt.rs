use exam_q2_lib::split_once;

fn main() {
    let string = String::from("hello world");
    let tuple: Option<(&str, &str)> = split_once(string.as_str(), " ");
    
    println!("{tuple:?}");
}
