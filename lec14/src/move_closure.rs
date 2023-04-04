pub fn main() {
    // let my_string = "hello";
    let my_string = String::from("hello");

    std::thread::spawn(move || {
        println!("{}", my_string);
    }).join().unwrap();

    // println!("From main: {my_string}");
}

// struct MyClosure {
//     my_string: String,
// }
// 
// impl FnOnce for MyClosure {
//     fn call_once(self, args: Args) -> Self::Output {
//         let my_string = self.my_string;
//         println!("{my_string}");
//     }
// }
