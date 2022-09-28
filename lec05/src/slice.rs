fn first_word(string: &String) -> todo!() {
    
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
        assert_eq!(first_word(&String::from("thisisjustonebigword")), "thisisjustonebigword")
    }

    #[test]
    fn sentence() {
        assert_eq!(first_word(&String::from("many different words in a larger sentence")), "many")
    }
}
