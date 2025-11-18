use require_lifetimes::require_lifetimes;

#[require_lifetimes]
pub fn split_many_1<'a, 'b>(parts: &'a Vec<String>, split_on: &'b str) -> Vec<(&'a str, &'b str, &'a str)> {
    let mut many = vec![];

    for s in parts {
        if let Some(index) = s.find(split_on) {
            many.push((&s[..index], split_on, &s[index + split_on.len()..]))
        }
    }

    many
}

#[require_lifetimes]
pub fn split_many_2<'a, 'b>(parts: &'a Vec<String>, split_on: &'b str) -> Vec<(&'a str, &'a str, &'a str)> {
    let mut many = vec![];

    for s in parts {
        if let Some(index) = s.find(split_on) {
            many.push((&s[..index], &s[index .. index + split_on.len()], &s[index + split_on.len()..]))
        }
    }

    many
}
