fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}

/// ```
/// split_string("hello world my name is zac", " ") ->
///     vec!["hello", "world", "my", "name", "is", "zac"]
/// ```
fn split_string<'a>(big_string: &'a str, split_on: &str) -> Vec<&'a str> {
    big_string.split(split_on).collect()
}

// fn longest_string<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
// where
//     // where I specify lifetime constraints
//     'a: 'c,
//     'b: 'c,
// {
//     if x.chars().count() >= y.chars().count() {
//         x
//     } else {
//         y
//     }
// }
