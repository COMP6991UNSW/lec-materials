pub fn split_once<'a, 'b>(string: &'a str, split_on: &'b str) -> Option<(&'b str, &'a str)> {
    let index = string.find(split_on)?;
    let tuple = (&string[..index], &string[index + split_on.len()..]);

    Some(tuple)
}
