fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 'x: 'y

fn foo() {
    let long_lifetime_string = String::from("hello"); // 'x

    {
        {
            {
                let very_short_lifetime_string = String::from("foo bar baz"); // 'y

                // let longer = longest(&long_lifetime_string, &very_short_lifetime_string);
            }
            // ...
        }
        // ...
    }
    // ...
}














#[cfg(test)]
mod tests {
    #[test]
    fn can_we_misuse() {

    }
}
