use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let tokens: Vec<String> = line.split_whitespace().map(String::from).collect();
    
    // TODO: Implement your StackLang interpreter here!
    // 
    // After execution, print each value on the stack,
    // one per line, from bottom to top.
}
