macro_rules! my_vec {
    () => {
        Vec::new()
    };

    ($($elem:expr),+) => {
        {
            let mut the_vec = Vec::new();
            $(
                the_vec.push($elem);
            )+

            the_vec
        }
    };
}

pub fn main() {
    let xs: Vec<i32> = my_vec![];

    let xs = my_vec![1, 2, 3, 4, 5];
    println!("{xs:?}");
}
