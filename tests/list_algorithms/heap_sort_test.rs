//! Heap Sort Test Suite
//!
//! Comprehensive tests for the generic, production-grade heap_sort algorithm.

use lunaris_engine::list::heap_sort::heap_sort;

#[test]
fn test_heap_sort_basic() {
    let mut arr = vec![4, 10, 3, 5, 1];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![1, 3, 4, 5, 10]);
}

#[test]
fn test_heap_sort_empty() {
    let mut arr: Vec<i32> = vec![];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![]);
}

#[test]
fn test_heap_sort_single() {
    let mut arr = vec![42];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![42]);
}

#[test]
fn test_heap_sort_duplicates() {
    let mut arr = vec![5, 1, 5, 3, 5];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![1, 3, 5, 5, 5]);
}

#[test]
fn test_heap_sort_reverse() {
    let mut arr = vec![9, 8, 7, 6, 5];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![5, 6, 7, 8, 9]);
}

#[test]
fn test_heap_sort_strings() {
    let mut arr = vec!["pear", "apple", "orange", "banana"];
    heap_sort(&mut arr);
    assert_eq!(arr, vec!["apple", "banana", "orange", "pear"]);
}
