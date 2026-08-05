use std::collections::HashSet;

pub struct Processor {
    items: Vec<String>,
}

impl Processor {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn map_all(&self, f: fn(&String) -> String) -> Vec<String> {
        self.items.iter().map(f).collect()
    }

    pub fn filter_dedup(&self, pred: fn(&String) -> bool) -> Vec<String> {
        let mut seen = HashSet::new();
        self.items
            .iter()
            .filter(|x| pred(x) && seen.insert((*x).clone()))
            .cloned()
            .collect()
    }

    pub fn into_sorted(self, f: fn(Vec<String>) -> Vec<String>) -> Vec<String> {
        let mut result = f(self.items);
        result.sort();
        result
    }
}
