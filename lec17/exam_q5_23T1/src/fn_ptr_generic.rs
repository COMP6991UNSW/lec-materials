use exam_q5_lib::Cache;

fn int_to_string(x: &i32) -> String {
    println!("Function called!");
    x.to_string()
}

fn main() {
    let mut cache = Cache::new(int_to_string);
    println!("cache.get(&1) = {:?}", cache.get(1));
    println!("cache.get(&2) = {:?}", cache.get(2));
    println!("cache.get(&3) = {:?}", cache.get(3));
    println!("cache.get(&42) = {:?}", cache.get(42));
    println!("cache.get(&1) = {:?}", cache.get(1));
    println!("cache.get(&2) = {:?}", cache.get(2));
    println!("cache.get(&3) = {:?}", cache.get(3));
    println!("cache.get(&42) = {:?}", cache.get(42));
}
