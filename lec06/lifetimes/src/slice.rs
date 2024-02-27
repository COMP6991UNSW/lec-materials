fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;

    for elem in list {
        sum += elem;
    }

    sum
}

fn first_word(string: &str) -> &str {
    string.split_whitespace().next().unwrap_or(string)
}


// fn break_it() {
//     let mut vec = vec![1, 2, 3];
// 
//     for elem in vec {
//         vec.push(1);
//     }
// }




/// Returns the first half of the list (rounded down).
/// 
/// ```
/// assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
///
/// assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5, 6]), &[1, 2, 3]);
/// ```
fn first_half(elems: &[i32]) -> &[i32] {
    let halfway = elems.len() / 2;
    &elems[0..halfway]
}







#[cfg(test)]
mod tests {
    use super::{sum_list, first_word};

    #[test]
    fn test1() {
        assert_eq!(sum_list(&vec![1, 2, 3]), 6);
        assert_eq!(sum_list(&[1, 2, 3]), 6);
    }

    #[test]
    fn it_works() {
        assert_eq!(first_word(&String::from("hello world")), "hello")
    }

    #[test]
    fn one_word() {
        assert_eq!(first_word("thisisjustonebigword"), "thisisjustonebigword")
    }

    #[test]
    fn sentence() {
        assert_eq!(first_word("many different words in a larger sentence"), "many")
    }
}
