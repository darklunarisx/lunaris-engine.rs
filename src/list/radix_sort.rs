//! Radix Sort (Generic, Production-Grade, for unsigned integers)
//!
//! Sorts a mutable slice in ascending order using the Radix Sort algorithm (LSD, base 256).
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy` + `Ord` + `From<u8>` + `Into<u64>`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::radix_sort::radix_sort;
//! let mut arr = vec![170u32, 45, 75, 90, 802, 24, 2, 66];
//! radix_sort(&mut arr);
//! assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
//! ```
/// Radix Sort: O(d(n + b)) time, O(n + b) space, not stable.
/// Only works for unsigned integer types (u8, u16, u32, u64, usize).
pub fn radix_sort<T>(arr: &mut [T])
where
    T: Copy + Ord + Into<u64>,
{
    // No extra compile-time unsigned check needed; Into<u64> is not implemented for signed types.

    let n = arr.len();
    if n == 0 {
        return;
    }

    // Find the maximum value to know the number of digits
    let max = arr.iter().cloned().max().unwrap();
    let max_val = max.into();

    // Buffer for temporary storage; safe because n > 0
    let mut buf = vec![arr[0]; n];

    // Exponent / current byte position (base 256)
    let mut exp = 1u64;

    // Perform LSD radix sort, processing one byte (0..255) at a time
    while max_val / exp > 0 {
        // Count occurrence of each byte value [0..255]
        let mut count = [0usize; 256];
        for &val in arr.iter() {
            let byte = ((val.into() / exp) % 256) as usize;
            count[byte] += 1;
        }

        // Prefix sum to get positions
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Build output array (stable for this pass)
        for &val in arr.iter().rev() {
            let byte = ((val.into() / exp) % 256) as usize;
            count[byte] -= 1;
            buf[count[byte]] = val;
        }

        // Copy back to input slice
        arr.copy_from_slice(&buf);

        // Move to next byte
        exp = exp.saturating_mul(256);
    }
}
