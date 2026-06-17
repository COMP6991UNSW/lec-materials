struct Foo<'a, 'b> {
    string_1: &'a str,
    string_2: &'b str,
}

fn make_foo<'fred, 'george>(
    string_1: &'fred str,
    string_2: &'george str
) -> Foo<'fred, 'george> {
    Foo {
        string_1,
        string_2,
    }
}

fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where
    // 't outlives 'y
    // 't >= 'y
    // 't: 'y
    'a: 'c,
    'b: 'c,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_real<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// The output borrow ALWAYS borrows from `x`
fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

/// The output borrow ALWAYS borrows from `y`
fn second<'y>(x: &str, y: &'y str) -> &'y str {
    y
}

fn neither(x: &str, y: &str) -> &'static str {
    "hello!!!"
}

// lifetime elision
fn first_word(string: &str) -> &str {
    string.split_whitespace().next().unwrap_or(string)
}

fn main() {
    let long_lifetime_string = String::from("looooong!");
    let long_string_str = long_lifetime_string.as_str();

    {
        {

            {
                {
                    let l;
                    {
                        let short_lifetime_string = String::from("omg my lifetime is so short!!!");
                        let short_string_str = short_lifetime_string.as_str();

                        l = longest(
                            long_string_str,
                            short_string_str);
                    }
                    println!("{l}");
                }
                // ....
            }
            // ....
        }
    }
}



//  let my_str = String::from("whatever").as_str();
//  println!("{my_str}");
