fn main() {
    let vec = [3, 1, 2].into_iter()
        .map(|x| x * 2)
        .collect_sorted_vec();

    assert_eq!(vec, vec![2, 4, 6]);
}

trait CollectSortedVec<T> {
    fn collect_sorted_vec(self) -> Vec<T>;
}

impl<I, IterItem> CollectSortedVec<IterItem> for I
where
    I: Iterator<Item = IterItem>,
    IterItem: Ord,
    // <I as Iterator>::Item: Ord,
{
    fn collect_sorted_vec(self) -> Vec<IterItem> {
        let mut vec = self.collect::<Vec<_>>();
        vec.sort();
        vec
    }
}
