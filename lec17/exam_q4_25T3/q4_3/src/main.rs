macro_rules! repeat_vec {
    ($($count:literal of $elem:expr),*) => {
        {
            let mut vec = Vec::new();

            $(
                for _ in 0..$count {
                    vec.push($elem);
                }
            )*

            vec
        }
    };
}

fn main() {
    let v = repeat_vec![3 of 1, 2 of 2];
    // v is vec![1, 1, 1, 2, 2]

    let greetings = repeat_vec![2 of "hello", 1 of "world"];
    // greetings is vec!["hello", "hello", "world"]

    let single = repeat_vec![5 of 0];
    // single is vec![0, 0, 0, 0, 0]
    
    dbg!(v);
    dbg!(greetings);
    dbg!(single);
}
