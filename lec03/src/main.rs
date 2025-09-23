use std::option::Option;

// sum type
enum AusCoin {
    FiveCent,
    TenCent(i32), // 42
    TwentyCent(String), // "hello"
    FiftyCent,
    OneDollar,
    TwoDollar,
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    dbg!(sum(vec![1, 2, 3, 4]));
    dbg!(mean(vec![1, 2, 3, 4]).unwrap_or(0.0));
    dbg!(mean(vec![3, 1, 11]).unwrap_or(0.0));
    dbg!(mean(vec![]).unwrap_or(0.0));

    let my_option: Option<String> = None;
    let my_option: Option<String> = Some("hello".to_string());

    let tuple = (42, 123.456, 'c', "foo");
    println!("{tuple:?}");

    let first = tuple.0;
    let second = tuple.1;
    // for item in tuple {
    //     
    // }

    let array = [1, 2, 3, 4, 5];
    for item in array {
        println!("{item}");
    }
    let second_get = array.get(2);
    let second_index = array[2];

    match second_get {
        Some(value) => {
            println!("The 2nd element is: {}", value);
        }
        None => {
            println!("There is no second element");
        }
    }

    let x = match second_get {
        Some(value) => *value,
        None => -1,
    };
    let x = second_get.copied().unwrap_or(-1);

    println!("x is {x}");

    println!("The sum is: {}", sum_array(array));

    let coin1 = AusCoin::FiveCent;
    let coin2 = AusCoin::TenCent(42);
    let coin3 = AusCoin::TenCent(123);
    let coin4 = AusCoin::TenCent(5);
    let coin5 = AusCoin::TenCent(10);
}

fn sum_array(array: [i32; 5]) -> i32 {
    let mut sum = 0;
    for item in array {
        sum += item;
    }
    sum
}

fn sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vec {
        sum += item;
    }
    sum
}

fn mean(vec: Vec<i32>) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }
    
    let s = sum(vec.clone()) as f64;
    let average = s / vec.len() as f64;

    Some(average)
}

fn longest_equal_run(x: Vec<i32>, y: Vec<i32>) -> usize {
    todo!()
}

fn random_dice_roll() -> usize {
    // chosen by fair dice roll
    7
}
