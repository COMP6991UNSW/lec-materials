use std::collections::HashMap;

pub struct CountingMap {
    map: HashMap<String, i32>,
}

impl CountingMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, k: String) -> &mut i32 {
        self.map.entry(k).or_default()
    }

    pub fn add_to_key(&mut self, k: String, v: i32) {
        *self.get_mut(k) += v;
    }

    pub fn max_count(&self) -> Option<(String, i32)> {
        self.map
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
    }
}
