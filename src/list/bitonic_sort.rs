//! Bitonic Sort (Generic, Production-Grade, for power-of-two length)
//!
//! Sorts a mutable slice in ascending order using the Bitonic Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::bitonic_sort::bitonic_sort;
//! let mut arr = vec![3, 7, 4, 8, 6, 2, 1, 5];
//! bitonic_sort(&mut arr, true);
//! assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
//! ```
/// Bitonic Sort: O(n log^2 n) time, O(log n) space, not stable.
pub fn bitonic_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    fn bitonic_merge<T: Ord>(arr: &mut [T], ascending: bool) {
        let n = arr.len();
        if n <= 1 {
            return;
        }
        let m = n / 2;
        for i in 0..m {
            if (arr[i] > arr[i + m]) == ascending {
                arr.swap(i, i + m);
            }
        }
        bitonic_merge(&mut arr[..m], ascending);
        bitonic_merge(&mut arr[m..], ascending);
    }
    let n = arr.len();
    if n <= 1 { return; }
    let m = n / 2;
    bitonic_sort(&mut arr[..m], true);
    bitonic_sort(&mut arr[m..], false);
    bitonic_merge(arr, ascending);
}
