//! Stooge Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Stooge Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::stooge_sort::stooge_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! stooge_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Stooge Sort: O(n^{2.709}) time, O(1) space, not stable.
pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    fn stooge<T: Ord>(arr: &mut [T], l: usize, h: usize) {
        if l >= h { return; }
        if arr[l] > arr[h] {
            arr.swap(l, h);
        }
        if h - l + 1 > 2 {
            let t = (h - l + 1) / 3;
            stooge(arr, l, h - t);
            stooge(arr, l + t, h);
            stooge(arr, l, h - t);
        }
    }
    if arr.len() > 1 {
        stooge(arr, 0, arr.len() - 1);
    }
}
