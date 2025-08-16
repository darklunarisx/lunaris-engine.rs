//! Shell Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Shell Sort algorithm (gap sequence: Shell's original).
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::shell_sort::shell_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! shell_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Shell Sort: O(n^2) worst, O(n log n) best, O(1) space, not stable.
pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j, j - gap);
                j -= gap;
            }
        }
        gap /= 2;
    }
}
