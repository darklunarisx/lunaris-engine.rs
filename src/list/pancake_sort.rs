//! Pancake Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Pancake Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::pancake_sort::pancake_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! pancake_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Pancake Sort: O(n^2) time, O(1) space, not stable.
pub fn pancake_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for curr_size in (1..=n).rev() {
        let mut max_idx = 0;
        for i in 1..curr_size {
            if arr[i] > arr[max_idx] {
                max_idx = i;
            }
        }
        if max_idx != curr_size - 1 {
            arr[..=max_idx].reverse();
            arr[..curr_size].reverse();
        }
    }
}
