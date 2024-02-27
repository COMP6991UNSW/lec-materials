pub fn give_struct<'a>(s: &'a str) -> StructWithBorrow<'a> {
    StructWithBorrow {
        x: s,
    }
}

pub struct StructWithBorrow<'a> {
    x: &'a str,
}

enum EnumWithBorrow<'a> {
    Owned(String),
    Borrowed(&'a str),
}
