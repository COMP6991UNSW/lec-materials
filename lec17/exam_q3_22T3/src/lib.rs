pub struct DBMap<K, V> {
    pub data: Vec<(K, V)>
}

impl<K: Eq, V1> DBMap<K, V1> {
    pub fn merge<V2>(self, mut other: DBMap<K, V2>) -> DBMap<K, (V1, Option<V2>)> {
        let mut output = Vec::new();

        for (key1, value1) in self.data {
            let index = other.data.iter()
                .enumerate()
                .find(|(_, (key2, _))| key1 == *key2)
                .map(|(index, _)| index);

            let option_v2 = match index {
                Some(index) => {
                    let (_, value2) = other.data.remove(index);
                    Some(value2)
                }
                None => {
                    None
                }
            };

            output.push((key1, (value1, option_v2)));
        }

        DBMap { data: output }
    }
}
