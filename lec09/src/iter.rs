pub fn main() {
    // 1, 1, 2, 3, 5, 8, 13, 21, ...

    let fib = Fibonacci::new();

    for elem in fib.take(10) {
        println!("{elem}");
    }

    let my_vec = vec![1, 2, 3, 4, 5];
    let my_iter = MyVecIter::new(&my_vec);

    for item in my_iter {
        println!("{item}");
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            curr: 1,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let give_back = self.curr;

        let next_elem = self.curr + self.next;
        self.curr = self.next;
        self.next = next_elem;

        Some(give_back)
    }
}

struct MyVecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> MyVecIter<'a, T> {
    fn new(vec: &'a Vec<T>) -> Self {
        Self {
            vec,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_item = self.vec.get(self.index)?;
        self.index += 1;

        Some(curr_item)
    }
}