//! Comb Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Comb Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::comb_sort::comb_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! comb_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Comb Sort: O(n^2) worst, O(n log n) average, O(1) space, not stable.
pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n;
    let shrink = 1.3;
    let mut sorted = false;
    while !sorted {
        gap = (gap as f64 / shrink) as usize;
        if gap < 1 { gap = 1; }
        sorted = gap == 1;
        for i in 0..n - gap {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}
