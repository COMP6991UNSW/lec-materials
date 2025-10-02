/*
fn longest<'a, 'b, 'c>(x: &'a String, y: &'b String) -> &'c String
where
    'a: 'c,
    'b: 'c,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn give_first_string_back<'a>(x: &'a String, y: &String) -> &'a String {
    x
}

fn foo<'a, 'b>(x: &'a &'b String)
where
    'b: 'a,
{

}

fn caller() {
    let string1 = String::from("short string");
    let string1b = &string1;

    {
        let string3;

        {
            {
                let string2 = String::from("very very very long string");
                let string2b = &string2;

                string3 = longest(string1b, string2b);
                println!("{string3}");
            }

        }
    }

    println!("{string1b}");
}
