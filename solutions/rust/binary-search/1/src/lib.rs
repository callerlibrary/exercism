pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut array = array.to_vec();
    array.sort();

    binary_search_generic(&array, &key)
}

fn binary_search_generic<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}
