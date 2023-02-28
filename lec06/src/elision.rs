fn first_word(string: &str) -> &str {
    string.split_once(' ')
        .map(|(before, after)| before)
        .unwrap_or(string)
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(first_word("hello world"), "hello")
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