macro_rules! up_to {
    ($var:ident, $n:expr, $body:block) => {
        for $var in 0..$n { $body }
    }
}

pub fn test_up_to() {
    up_to!(foo, 10, {
        println!("{foo}");
    });
}
