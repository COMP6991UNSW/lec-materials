use require_lifetimes::require_lifetimes;
/// This function returns the input it is given.
/// You will need to annotate its lifetimes
/// (2 marks)
#[require_lifetimes]
pub fn identity(a: &i32) -> &i32 {
    a
}

/// This function swaps the two references it is given.
/// You will need to annotate its lifetimes
/// (4 marks)
#[require_lifetimes]
pub fn swap(a: &i32, b: &i32) -> (&i32, &i32) {
    (b, a)
}

//// This function returns the two references it is given in sorted order,
//// with the smallest one first.
//// (4 marks)
#[require_lifetimes]
pub fn sort_references(a: &i32, b: &i32) -> (&i32, &i32) {
    if *a > *b {
        (b, a)
    } else {
        (a, b)
    }
}
