#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum DiffResult<'a> {
    LeftOnly(&'a str),
    RightOnly(&'a str),
    Both(&'a str)
}

pub fn compare_rolls<'a>(left_roll: &'a str, right_roll: &'a str) -> Vec<DiffResult<'a>> {
    todo!()
}
