pub fn give_struct(s: &str) -> StructWithBorrow {
    StructWithBorrow {
        x: s,
    }
}

// pub fn give_struct<'a>(s: &'a str) -> StructWithBorrow<'a> {
//     StructWithBorrow {
//         x: s,
//     }
// }

// TODO: Change to borrow
pub struct StructWithBorrow<'a> {
    x: &'a str,
}

// TODO: Add borrowed case
enum EnumWithBorrow<'a> {
    Owned(&'a str),
}
