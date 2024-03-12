pub fn main() {
    // dyn Animal

    let v = vec![1, 2, 3, 4, 5];
    let iter = MyVecIter::new(&v);

    drop(v);

    for item in iter.cloned().map(|x| x * 2).filter(|x| *x > 5) {
        println!("{item}");
    }
}

struct MyVecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> MyVecIter<'a, T> {
    fn new(vec: &'a Vec<T>) -> MyVecIter<'a, T> {
        Self {
            vec,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let value = self.vec.get(self.index)?;
        self.index += 1;
        Some(value)
    }
}
