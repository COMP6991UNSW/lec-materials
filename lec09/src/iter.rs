pub fn main() {
    println!("The first ten terms of the Fibonacci sequence are: ");
    for line in fibonacci().take(10) {
        println!("{}", line);
    }

    let my_vec = vec![
        String::from("Dong"),
        String::from("Simon"),
        String::from("Miguel"),
    ];
    for item in my_vec_iter(&my_vec) {
        println!("{item}");
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        curr: 1,
        next: 1,
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        let next_should_be = self.curr + self.next;

        self.curr = self.next;
        self.next = next_should_be;

        Some(curr)
    }
}



struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

fn my_vec_iter<'a, T>(vec: &'a Vec<T>) -> VecIter<'a, T> {
    VecIter {
        vec: vec,
        index: 0,
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

