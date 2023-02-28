fn longest_length<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn give_me_back_x_if_x_is_longer_than_y_otherwise_an_empty_string_will_do<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        ""
    }
}

// fn longest_length<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
// where
//     'a: 'c,
//     'b: 'c,
// {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(longest_length("hello", "zac"), "hello");
        assert_eq!(longest_length("zac", "hello"), "hello");

        let long_lived_string = String::from("Hello");

        {
            {
                {
                    {
                        {
                            let tiny_life_string = String::from("world!!");

                            let longest = longest_length(&long_lived_string, &tiny_life_string);
                            
                            assert_eq!(longest, "world!!");
                        }
                    }
                }
            }
        }
    }
}