fn sum_array(list: [i32; 10]) -> i32 {
    let mut sum = 0;

    for elem in list {
        sum += elem;
    }

    sum
}

fn sum_vec(list: Vec<i32>) -> i32 {
    let mut sum = 0;

    for elem in list {
        sum += elem;
    }

    sum
}
