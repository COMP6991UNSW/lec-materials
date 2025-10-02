fn string_chars_len(string: &String) -> usize {
    string.chars().count()
}




























#[cfg(test)]
mod tests {
    use super::string_chars_len;

    #[test]
    fn empty() {
        assert_eq!(string_chars_len(&String::from("")),     0);
    }

    #[test]
    fn ascii() {
        let my_string = String::from("a");
        assert_eq!(string_chars_len(&my_string),    1);
        println!("{my_string}");

        assert_eq!(string_chars_len(&String::from("ab")),   2);
        assert_eq!(string_chars_len(&String::from("abc")),  3);
        assert_eq!(string_chars_len(&String::from("abcd")), 4);
    }

    #[test]
    fn emoji() {
        assert_eq!(string_chars_len(&String::from("😀😃😄😁😆")), 5);
    }
}
