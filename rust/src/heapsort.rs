use std::collections::BinaryHeap;

#[allow(dead_code)]
fn heapsort<T: Ord + Clone>(collection: &[T]) -> Vec<T> {
    let mut heap: BinaryHeap<T> = BinaryHeap::with_capacity(collection.len());
    for n in collection {
        heap.push(n.clone());
    }

    heap.into_sorted_vec()
}

#[cfg(test)]
mod test {
    use super::heapsort;

    #[test]
    fn test_heapsort() {
        let mut numbers = [500, 213, 1, 300, 423];
        numbers.sort_unstable();
        assert_eq!(heapsort(&[500, 213, 1, 300, 423]), numbers);
    }
}
