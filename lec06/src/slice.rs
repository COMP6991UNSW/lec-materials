// elide
// elision
fn first_word<'a>(string: &'a str) -> &'a str {
    let mut end_index = 0;
    let bytes = string.as_bytes();

    for (index, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            break;
        }

        end_index += 1;
    }

    &string[..end_index]
}

#[cfg(test)]
mod tests {
    use super::first_word;

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
