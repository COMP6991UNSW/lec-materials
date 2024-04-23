use exam_q5_lib::Cache;

fn main() {
    let mut num = 0;
    let mut cache = Cache::new(|key| {
        println!("Closure called!");

        let value = format!("{key} {num}");
        num += 1;

        value
    });

    let foo = "foo";
    let bar = "bar";
    let baz = "baz";

    println!("cache.get(\"foo\") = {:?}", cache.get(foo));
    println!("cache.get(\"bar\") = {:?}", cache.get(bar));
    println!("cache.get(\"foo\") = {:?}", cache.get(foo));
    println!("cache.get(\"baz\") = {:?}", cache.get(baz));
    println!("cache.get(\"foo\") = {:?}", cache.get(foo));
    println!("cache.get(\"bar\") = {:?}", cache.get(bar));
    println!("cache.get(\"baz\") = {:?}", cache.get(baz));
}
