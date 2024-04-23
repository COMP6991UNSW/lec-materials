pub fn get_factors(mut num: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    for i in 2..=num {
        while num % i == 0 {
            factors.push(i);
            num = num / i;
        }
        if num == 1 {
            break;
        }
    }

    factors
}

pub fn get_common_factors(list1: &Vec<u128>, list2: &Vec<u128>) -> Vec<u128> {
    let mut list2 = list2.clone();
    let mut factors = Vec::new();

    for elem in list1 {
        let index = list2.iter().position(|x| *x == *elem);
        if let Some(i) = index {
            factors.push(*elem);
            list2.swap_remove(i);
        }
    }

    factors
}
