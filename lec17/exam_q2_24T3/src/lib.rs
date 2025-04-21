use std::collections::HashMap;

/// This function correctly implements finding a string in a list of strings.
/// You do not need to change this function at all, and can assume it behaves correctly.
///
/// If the string `s` exists in the list, it returns a reference to the first matching string in the list.
/// If the string `s` does not exist in the list, it returns None.
fn find_str_correct<'a, 'b, 'c>(l: &'a Vec<&'b str>, s: &'c str) -> Option<&'b str> {
    for x in l.iter() {
        if *x == s {
            return Some(x);
        }
    }

    None
}

/// By default, we will use this function. This function has the correct lifetimes.
#[cfg(not(feature = "find_str_bad_lifetimes"))]
pub fn find_str<'a, 'b, 'c>(l: &'a Vec<&'b str>, s: &'c str) -> Option<&'b str> {
    find_str_correct(l, s)
}

/// If you compile with `cargo build --feature find_str_bad_lifetimes`, this function will be used instead.
/// This function has incorrect lifetimes, and your job is to ensure that when your code calls this function,
/// it does not compile.
#[cfg(feature = "find_str_bad_lifetimes")]
pub fn find_str<'a, 'b>(l: &'a Vec<&'b str>, s: &'b str) -> Option<&'b str> {
    find_str_correct(l, s)
}

/// This function correctly implements finding the most common string in a list of strings.
/// You do not need to change this function at all, and can assume it behaves correctly.
///
/// It will return the most common string in the list, or None if the list is empty.
/// If there are multiple strings that are equally common, it will return the first one it encounters.
fn most_common_correct<'a, 'b>(l: &'a Vec<&'b str>) -> Option<&'b str> {
    let mut counts = HashMap::new();
    for x in l.iter() {
        *counts.entry(x).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut max_str = None;
    for (s, count) in counts {
        if count > max_count {
            max_count = count;
            max_str = Some(*s);
        }
    }

    max_str
}

/// By default, we will use this function. This function has the correct lifetimes.
#[cfg(not(feature = "most_common_bad_lifetimes"))]
pub fn most_common<'a, 'b>(l: &'a Vec<&'b str>) -> Option<&'b str> {
    most_common_correct(l)
}

/// If you compile with `cargo build --feature most_common_bad_lifetimes`, this function will be used instead.
/// This function has incorrect lifetimes, and your job is to ensure that when your code calls this function,
/// it does not compile.
#[cfg(feature = "most_common_bad_lifetimes")]
pub fn most_common<'a, 'b>(l: &'b Vec<&'b str>) -> Option<&'b str> {
    most_common_correct(l)
}
