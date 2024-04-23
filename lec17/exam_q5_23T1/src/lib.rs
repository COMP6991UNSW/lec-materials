use std::{collections::HashMap, rc::Rc};

pub struct Cache {
    calculator: fn(&String) -> String,
    cache_map: HashMap<String, Rc<String>>,
}

impl Cache {
    pub fn new(calculator: fn(&String) -> String) -> Self {
        Cache {
            calculator,
            cache_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: String) -> Rc<String> {
        if let Some(value) = self.cache_map.get(&key) {
            Rc::clone(value)
        } else {
            let value = Rc::new((self.calculator)(&key));
            self.cache_map.insert(key, Rc::clone(&value));

            value
        }
    }
}
