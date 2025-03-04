// fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
//     where
//         'a: 'c,
//         'b: 'c,
// {
//     if x.len() >= y.len() {
//         x
//     } else {
//         y
//     }
// }

// struct Longest<'a> {
//     longest: &'a str,
// }
// 
// fn longest<'a>(x: &'a str, y: &'a str) -> Longest<'a> {
//     if x.len() >= y.len() {
//         Longest { longest: x }
//     } else {
//         Longest { longest: y }
//     }
// }


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

fn gives_back(x: &str) -> &'static str {
    "hello"
}

// fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'b str
//     where
//         'a: 'b,
// {
//     x
// }

fn main() {
    let longer;
    let firster;

    let long_lifetime_string = String::from("hello");

    {

        {
            {
                let short_lifetime_string = String::from("world!");

                longer = longest(&long_lifetime_string, &short_lifetime_string);
                firster = first(&long_lifetime_string, &short_lifetime_string);
                println!("{longer}");
            }
        }


        println!("{firster}");
    }
}
