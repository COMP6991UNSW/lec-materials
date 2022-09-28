/// # Emphasize
/// 
/// Emphasize should take a `String`,
/// and mutate the following:
/// 
/// * Make any ASCII characters uppercase
/// * Append "!!!" to the end
/// 
/// For example:
/// 
/// ```
/// let mut s = String::from("test");
/// emphasize(&mut s);
///
/// assert_eq!(s, "TEST!!!");
/// ```
/// Google: noalias (Rust)
fn emphasize(string: &mut String) {
    string.make_ascii_uppercase();
    string.push_str("!!!");
}

#[cfg(test)]
mod tests {
    use super::emphasize;

    #[test]
    fn empty() {
        let mut s = String::from("");
        emphasize(&mut s);

        assert_eq!(s, "!!!");
    }

    #[test]
    fn ascii() {
        let mut s = String::from("a");
        emphasize(&mut s);
        assert_eq!(s, "A!!!");

        let mut s = String::from("aB");
        let excl_borrow_1 = &mut s;
        emphasize(excl_borrow_1);
        assert_eq!(s, "AB!!!");
        
        let mut s = String::from("aBc");
        emphasize(&mut s);

        assert_eq!(s, "ABC!!!");
        
        let mut s = String::from("Hello, World!");
        emphasize(&mut s);

        assert_eq!(s, "HELLO, WORLD!!!!");
    }

    #[test]
    fn emoji() {
        let mut s = String::from("😀😃😄😁😆");
        emphasize(&mut s);

        assert_eq!(s, "😀😃😄😁😆!!!");
    }
}
