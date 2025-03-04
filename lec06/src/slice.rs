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

fn sum_slice(list: &[i32]) -> i32 {
    let mut sum = 0;

    for elem in list {
        sum += elem;
    }

    sum
}


fn first_word(string: &str) -> &str {
    string.split_whitespace().next().unwrap_or(string)
}





#[cfg(test)]
mod tests {
    use super::{sum_slice, first_word};

    #[test]
    fn test1() {
        let array = [1, 2, 3, 4, 5];
        let slice = &array[1..4];

        assert_eq!(sum_slice(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
        assert_eq!(sum_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
        assert_eq!(sum_slice(slice), 9);
    }
}
