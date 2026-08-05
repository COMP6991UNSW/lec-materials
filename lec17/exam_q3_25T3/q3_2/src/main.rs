fn process<T>(items: Vec<T>) -> Option<T>
where
    T: Ord,
{
    items.into_iter().max()
}

fn main() {}
