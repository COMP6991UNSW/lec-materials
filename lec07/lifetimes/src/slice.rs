fn first_word<'a>(string: &str) -> &str {
    string.split_whitespace().next().unwrap_or("")
}





/// Returns the first half of the list (rounded down).
/// 
/// ```
/// assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
///
/// assert_eq!(first_half(&[1, 2, 3, 4]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5]), &[1, 2]);
/// assert_eq!(first_half(&[1, 2, 3, 4, 5, 6]), &[1, 2, 3]);
/// ```
fn first_half<'a, 'b, 'c, T, U, V>(elems: &[T]) -> &[T] {
    let halfway = elems.len() / 2;
    let half_slice = &elems[..halfway];

    half_slice
}







#[cfg(test)]
mod tests {
    use super::first_word;

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
