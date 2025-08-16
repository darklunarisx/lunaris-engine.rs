//! Quickselect (Generic, Production-Grade)
//!
//! Finds the k-th smallest element in a mutable slice using the Quickselect algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::quickselect::quickselect;
//! let mut arr = vec![7, 10, 4, 3, 20, 15];
//! let kth = quickselect(&mut arr, 2);
//! // The 2nd smallest is 4, but quickselect is not stable; sort to check:
//! let mut arr2 = vec![7, 10, 4, 3, 20, 15];
//! arr2.sort();
//! assert_eq!(kth, Some(arr2[2]));
//! ```
/// Quickselect: O(n) average, O(n^2) worst, O(1) space, not stable.
use rand::Rng;
pub fn quickselect<T: PartialOrd + Copy>(arr: &mut [T], k: usize) -> Option<T> {
    if arr.is_empty() || k >= arr.len() {
        return None;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut rng = rand::rng();
    while left <= right {
        let pivot_idx = rng.random_range(left..=right);
        arr.swap(pivot_idx, right);
        let pivot = partition(arr, left, right);
        if pivot == k {
            return Some(arr[pivot]);
        } else if pivot > k {
            if pivot == 0 {
                break;
            }
            right = pivot - 1;
        } else {
            left = pivot + 1;
        }
    }
    None
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;
    for j in left..right {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}
