//! Odd-Even Sort (Brick Sort, Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Odd-Even (Brick) Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::odd_even_sort::odd_even_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! odd_even_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Odd-Even Sort: O(n^2) time, O(1) space, stable.
pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut sorted = false;
    while !sorted {
        sorted = true;
        // Odd phase
        for i in (1..n - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        // Even phase
        for i in (0..n - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}
