/// <https://codereview.stackexchange.com/questions/145113/bucket-sort-in-rust/>
struct Bucket<H, V> {
    hash: H,
    values: Vec<V>,
}

impl<H, V> Bucket<H, V> {
    fn new(hash: H, value: V) -> Bucket<H, V> {
        Bucket {
            hash,
            values: vec![value],
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn bucket_sort<T, F, H>(values: Vec<T>, hasher: F) -> Vec<T>
where
    T: Ord,
    F: Fn(&T) -> H,
    H: Ord,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for value in values {
        let hash = hasher(&value);
        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(value),
            Err(index) => buckets.insert(index, Bucket::new(hash, value)),
        }
    }

    buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect()
}

#[test]
fn test_bucket_sort() {
    let values = vec![5, 10, 2, 99, 32, 1, 7, 9, 92, 135, 0, 54];
    let sorted_values = bucket_sort(values, |int| int / 10);
    assert_eq!(sorted_values, [0, 1, 2, 5, 7, 9, 10, 32, 54, 92, 99, 135]);
}
