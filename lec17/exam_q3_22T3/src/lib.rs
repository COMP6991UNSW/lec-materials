pub struct DBMap {
    pub data: Vec<(i32, &'static str)>
}

impl DBMap {
    pub fn merge(self, mut other: DBMap) -> DBMap {
        todo!()
    }
}
