fn main() {
    let my_vec = vec![1, 2, 3];
    let iter = MyVecIter::from(&my_vec);

    for item in iter.map(|x| x * 2) {
        println!("{item}");
    }
}

impl<'a, T> From<&'a Vec<T>> for MyVecIter<'a, T> {
    fn from(vec: &'a Vec<T>) -> Self {
        MyVecIter { vec, index: 0 }
    }
}

struct MyVecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.index)?;
        self.index += 1;
        Some(item)
    }
}
