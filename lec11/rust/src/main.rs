mod sort;

macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($item:expr),+$(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($item);
            )+
            v
        }
    };
}

fn main() {
    let my_vec: Vec<i32> = my_vec![];
    println!("{my_vec:?}");

    let my_vec: Vec<i32> = my_vec![
        1,
        2,
        3,
    ];
    println!("{my_vec:?}");

    let std_vec: Vec<i32> = vec![
        1,
        2,
        3,
    ];
    println!("{std_vec:?}");
}
