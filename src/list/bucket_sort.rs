//! Bucket Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Bucket Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy` + `PartialOrd` + `From<f64>` + `Into<f64>`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::bucket_sort::bucket_sort;
//! let mut arr = vec![0.42, 4.21, 3.14, 2.71, 1.61];
//! bucket_sort(&mut arr);
//! assert_eq!(arr, vec![0.42, 1.61, 2.71, 3.14, 4.21]);
//! ```
/// Bucket Sort: O(n + k) expected, O(n) space, not stable.
pub fn bucket_sort<T>(arr: &mut [T])
where
    T: Copy + PartialOrd + From<f64> + Into<f64>,
{
    let n = arr.len();
    if n == 0 {
        return;
    }
    let min = arr.iter().cloned().fold(arr[0], |a, b| if a < b { a } else { b });
    let max = arr.iter().cloned().fold(arr[0], |a, b| if a > b { a } else { b });
    let bucket_count = n;
    let mut buckets: Vec<Vec<T>> = vec![Vec::new(); bucket_count];
    let range = max.into() - min.into();
    for &val in arr.iter() {
        let idx = if range == 0.0 {
            0
        } else {
            (((val.into() - min.into()) / range) * (bucket_count as f64 - 1.0)).round() as usize
        };
        buckets[idx].push(val);
    }
    let mut idx = 0;
    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for &val in bucket.iter() {
            arr[idx] = val;
            idx += 1;
        }
    }
}
