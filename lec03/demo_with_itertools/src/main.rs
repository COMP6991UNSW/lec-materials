use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

fn longest_equal_super_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    x.into_iter().zip(y.into_iter())
        .map(|(x, y)| x == y)
        .group_by(|x| *x)
        .into_iter()
        .filter(|(v, _)| *v)
        .map(|(_v, l)| l.count())
        .max()
        .unwrap_or(0)
}
