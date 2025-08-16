//! Heap Sort (Generic, Production-Grade)
//!
//! Sorts a mutable slice in ascending order using the Heap Sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Example
//! ```rust
//! use lunaris_engine::list::heap_sort::heap_sort;
//! let mut arr = vec![4, 10, 3, 5, 1];
//! heap_sort(&mut arr);
//! assert_eq!(arr, vec![1, 3, 4, 5, 10]);
//! ```
/// Heap Sort: O(n log n) time, O(1) space, not stable.
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify_iter(arr, n, i);
    }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify_iter(arr, i, 0);
    }
}

fn heapify_iter<T: PartialOrd>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        if l < n && arr[l] > arr[largest] {
            largest = l;
        }
        if r < n && arr[r] > arr[largest] {
            largest = r;
        }
        if largest == i {
            break;
        }
        arr.swap(i, largest);
        i = largest;
    }
}
