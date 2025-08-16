//! Gnome Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Gnome Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::gnome_sort::gnome_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! gnome_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Gnome Sort: O(n^2) time, O(1) space, stable.
pub fn gnome_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut i = 0;
    while i < n {
        if i == 0 || arr[i] >= arr[i - 1] {
            i += 1;
        } else {
            arr.swap(i, i - 1);
            i -= 1;
        }
    }
}
