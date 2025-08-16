//! LIS (Longest Increasing Subsequence) with Binary Search (O(n log n), Generic, Production-Grade)
//!
//! Finds the length of the longest increasing subsequence in a slice using Patience Sorting (binary search).
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord` + `Copy`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::lis_binary_search::lis_length;
//! let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];
//! let len = lis_length(&arr);
//! assert_eq!(len, 4);
//! ```
/// LIS with Binary Search: O(n log n) time, O(n) space.
pub fn lis_length<T: PartialOrd + Copy>(arr: &[T]) -> usize {
    let mut tails = Vec::with_capacity(arr.len());
    for &x in arr {
        let mut left = 0;
        let mut right = tails.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if tails[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == tails.len() {
            tails.push(x);
        } else {
            tails[left] = x;
        }
    }
    tails.len()
}
