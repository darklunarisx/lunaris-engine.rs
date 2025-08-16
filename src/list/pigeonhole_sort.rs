//! Pigeonhole Sort (Generic, Production-Grade, for integers)
//!
//! Sorts a mutable slice in ascending order using the Pigeonhole Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy` + `Ord` + `From<i32>` + `Into<i32>`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::pigeonhole_sort::pigeonhole_sort;
//! let mut arr = vec![8, 3, 2, 7, 4, 6, 8, 5];
//! pigeonhole_sort(&mut arr);
//! assert_eq!(arr, vec![2, 3, 4, 5, 6, 7, 8, 8]);
//! ```
/// Pigeonhole Sort: O(n + range) time, O(range) space, stable.
pub fn pigeonhole_sort<T>(arr: &mut [T])
where
    T: Copy + Ord + From<i32> + Into<i32>,
{
    let n = arr.len();
    if n == 0 {
        return;
    }
    let min = arr.iter().map(|&x| x.into()).min().unwrap();
    let max = arr.iter().map(|&x| x.into()).max().unwrap();
    let size = (max - min + 1) as usize;
    let mut holes = vec![0usize; size];
    for &x in arr.iter() {
        holes[(x.into() - min) as usize] += 1;
    }
    let mut idx = 0;
    for i in 0..size {
        for _ in 0..holes[i] {
            arr[idx] = T::from(i as i32 + min);
            idx += 1;
        }
    }
}
