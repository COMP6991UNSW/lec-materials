pub fn foo<'a>(s: &'a String) -> StructWithBorrow<'a> {
    StructWithBorrow {
        x: s
    }
}

pub struct StructWithBorrow<'a> {
    x: &'a String,
}

enum EnumWithBorrow<'a> {
    Owned(String),
    Borrowed(&'a String),
}
