//! 🪄 Merge Sort (Generic, Stable)
//!
//! Sorts a vector in ascending order using the merge sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord + Clone`.
//!
//! # Arguments
//! * `vec` - The vector to sort (will be sorted in-place).
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::merge_sort::merge_sort;
//! let mut arr = vec![5, 2, 4, 6, 1, 3];
//! merge_sort(&mut arr);
//! assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
//! ```
pub fn merge_sort<T: Ord + Clone>(slice: &mut [T]) {
    let n = slice.len();
    if n <= 1 { return; }
    let mid = n / 2;
    let mut left = slice[..mid].to_vec();
    let mut right = slice[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(slice, &left, &right);
}

fn merge<T: Ord + Clone>(slice: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            slice[k] = left[i].clone();
            i += 1;
        } else {
            slice[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        slice[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        slice[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
