use std::{collections::HashSet, hash::Hash};

pub struct Processor<T> {
    items: Vec<T>,
}

impl<T> Processor<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn map_all<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U> {
        self.items.iter().map(f).collect()
    }

    pub fn filter_dedup(&self, mut pred: impl FnMut(&T) -> bool) -> Vec<T>
    where
        T: Hash + Eq + Clone,
    {
        let mut seen = HashSet::new();
        self.items
            .iter()
            .filter(|x| pred(x) && seen.insert((*x).clone()))
            .cloned()
            .collect()
    }

    pub fn into_sorted<U>(self, f: impl FnOnce(Vec<T>) -> Vec<U>) -> Vec<U>
    where
        U: Ord,
    {
        let mut result = f(self.items);
        result.sort();
        result
    }
}
