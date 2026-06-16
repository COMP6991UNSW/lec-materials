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
        assert_eq!(string_chars_len(&String::from("a")),    1);
        assert_eq!(string_chars_len(&String::from("ab")),   2);
        assert_eq!(string_chars_len(&String::from("abc")),  3);
        assert_eq!(string_chars_len(&String::from("abcd")), 4);
    }

    #[test]
    fn emoji() {
        assert_eq!(string_chars_len(&String::from("😀😃😄😁😆")), 5);
    }

    // #[test]
    // fn extra_case() {
    //     let mut string = String::from("hello");
    //     let count = string_chars_len(string);

    //     string.push_str(" world");
    //     assert_eq!(string, "hello world");
    // }
}
