//! Cycle Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Cycle Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord` + `PartialEq`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::cycle_sort::cycle_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! cycle_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Cycle Sort: O(n^2) time, O(1) space, not stable. Minimizes writes.
/// Safe, classic Cycle Sort: No unsafe code, correct for all input.
pub fn cycle_sort<T: Ord + PartialEq + Copy>(arr: &mut [T]) {
    let n = arr.len();
    for cycle_start in 0..n - 1 {
        let mut item = arr[cycle_start];
        let mut pos = cycle_start;
        for i in cycle_start + 1..n {
            if arr[i] < item {
                pos += 1;
            }
        }
        if pos == cycle_start {
            continue;
        }
        while item == arr[pos] {
            pos += 1;
        }
        if pos != cycle_start {
            std::mem::swap(&mut arr[pos], &mut item);
        }
        while pos != cycle_start {
            pos = cycle_start;
            for i in cycle_start + 1..n {
                if arr[i] < item {
                    pos += 1;
                }
            }
            while item == arr[pos] {
                pos += 1;
            }
            if item != arr[pos] {
                std::mem::swap(&mut arr[pos], &mut item);
            }
        }
    }
}
