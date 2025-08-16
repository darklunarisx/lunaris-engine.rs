//! Maximum Product Subarray (Generic, Production-Grade)
//!
//! Finds the maximum product of a contiguous subarray in a slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy` + `PartialOrd` + `Mul<Output = T>` + `One` + `Zero`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::maximum_product_subarray::maximum_product_subarray;
//! let arr = vec![2, 3, -2, 4];
//! let max_prod = maximum_product_subarray(&arr);
//! assert_eq!(max_prod, 6);
//! ```
use num_traits::{One, Zero};
// Manual max/min for PartialOrd
fn max3<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let mut m = a;
    if b > m {
        m = b;
    }
    if c > m {
        m = c;
    }
    m
}
fn min3<T: PartialOrd + Copy>(a: T, b: T, c: T) -> T {
    let mut m = a;
    if b < m {
        m = b;
    }
    if c < m {
        m = c;
    }
    m
}
use std::ops::Mul;

/// Maximum Product Subarray: O(n) time, O(1) space. Handles all-zero/all-negative arrays robustly.
pub fn maximum_product_subarray<T>(arr: &[T]) -> T
where
    T: Copy + PartialOrd + Mul<Output = T> + One + Zero,
{
    if arr.is_empty() {
        return T::zero();
    }
    let mut max_prod = arr[0];
    let mut min_prod = arr[0];
    let mut result = arr[0];
    for &num in &arr[1..] {
        let (a, b, c) = (num, max_prod * num, min_prod * num);
        max_prod = max3(a, b, c);
        min_prod = min3(a, b, c);
        result = if max_prod > result { max_prod } else { result };
    }
    result
}
