pub fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let iter = VecIter::new(&my_vec);

    for item in iter.map(|x| *x * 2).filter(|x| x % 3 != 0) {
        println!("Item: {item}");
    }

    // let mut first = 1;
    // let mut second = 1;

    // for _ in 0..30 {
    //     println!("{first}");
    //     let third = first + second;
    //     first = second;
    //     second = third;
    // }

    let fib_iter = FibIter::new();

    for elem in fib_iter.take(30) {
        println!("{elem}");
    }

    let mut fib_iter = FibIter::new();
    println!("{:?}", fib_iter.nth(40));

    // for item in iter {
    //     println!("Item: {item}");
    // }

    let array = ["foo", "bar", "baz"];
    let nats = 0..;

    for (name, number) in array.into_iter().zip(nats) {
        println!("{number}: {name}");
    }
}

struct FibIter {
    first: u64,
    second: u64,
}

impl FibIter {
    fn new() -> Self {
        Self {
            first: 1,
            second: 1,
        }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.first;

        let third = self.first + self.second;
        self.first = self.second;
        self.second = third;

        Some(return_value)
    }
}

struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> VecIter<'a, T> {
    fn new(vec: &'a Vec<T>) -> Self {
        Self {
            vec,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.index)?;
        self.index += 1;
        
        Some(item)
    }
}
