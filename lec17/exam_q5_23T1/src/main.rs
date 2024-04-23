use exam_q5_lib::Cache;

fn string_to_string(x: &String) -> String {
    println!("Function called!");

    x.chars().rev().collect()
}

fn main() {
    let mut cache = Cache::new(string_to_string);
    
    let foo = String::from("foo");
    let bar = String::from("bar");
    let baz = String::from("baz");

    println!("cache.get(foo) = {:?}", cache.get(foo.clone()));
    println!("cache.get(bar) = {:?}", cache.get(bar.clone()));
    println!("cache.get(baz) = {:?}", cache.get(baz.clone()));
    println!("cache.get(foo) = {:?}", cache.get(foo));
    println!("cache.get(baz) = {:?}", cache.get(baz));
    println!("cache.get(bar) = {:?}", cache.get(bar));
}
