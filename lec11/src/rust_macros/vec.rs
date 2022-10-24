macro_rules! vec {
    // vec![]
    () => {
        Vec::new()
    };
    // vec![1, 2, 3, ...]
    ($($item:expr),+ $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($item);
            )+
            vec
        }
    };
}

pub fn test_vec() {
    let my_vec: Vec<i32> = vec![42, 43, 44];

    // let my_vec: Vec<i32> = {
    //     let mut vec = Vec::new();
    //     vec.push(123);
    //     vec
    // };

    assert_eq!(std::vec![] as Vec<i32>, vec![]);

    assert_eq!(std::vec![1], vec![1]);

    assert_eq!(std::vec![1, 2], vec![1, 2]);
    assert_eq!(std::vec![1, 2, 3], vec![1, 2, 3]);

    assert_eq!(
        std::vec![
            1,
            2,
            3,
            4,
            5,
        ],
        vec![
            1,
            2,
            3,
            4,
            5,
        ],
    );

    // Should not compile!
    // vec![,]
}
