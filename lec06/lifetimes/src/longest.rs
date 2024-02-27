fn foo() {
    let really_long_lifetime: &'static str = "abc";
    
    let result;
    {
        {
            {
                {
                    let really_short_lifetime = String::from("fooooo");
                    let borrow_that = really_short_lifetime.as_str();

                    result = longest_string_2(really_long_lifetime, borrow_that);
                }

            }
        }
    }
    println!("{result}");
    
}

// fn get_or_default<T>(option: Option<T>) -> T
// where
//     T: Default,
// {
//     match option {
//         Some(value) => value,
//         None => Default::default(),
//     }
// }

fn longest_string<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where
    // outlives relations
    // foo outlives bar
    'a: 'c,
    'b: 'c,
{
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}

fn longest_string_2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}
