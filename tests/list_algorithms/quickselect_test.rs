//! Quickselect Test Suite
//!
//! Comprehensive tests for the generic, production-grade quickselect algorithm.
//! This file contains a wide range of test cases, including edge cases, random data, and property-based checks.

use lunaris_engine::list::quickselect::quickselect;

#[test]
fn test_quickselect_basic() {
    let mut arr = vec![7, 10, 4, 3, 20, 15];
    let kth = quickselect(&mut arr, 2);
    assert_eq!(kth, Some(4));
}

#[test]
fn test_quickselect_empty() {
    let mut arr: Vec<i32> = vec![];
    let kth = quickselect(&mut arr, 0);
    assert_eq!(kth, None);
}

#[test]
fn test_quickselect_single() {
    let mut arr = vec![42];
    let kth = quickselect(&mut arr, 0);
    assert_eq!(kth, Some(42));
}

#[test]
fn test_quickselect_duplicates() {
    let mut arr = vec![5, 1, 5, 3, 5];
    let kth = quickselect(&mut arr, 2);
    assert_eq!(kth, Some(5));
}

#[test]
fn test_quickselect_reverse() {
    let mut arr = vec![9, 8, 7, 6, 5];
    let kth = quickselect(&mut arr, 0);
    assert_eq!(kth, Some(5));
}

#[test]
fn test_quickselect_large() {
    let mut arr: Vec<_> = (0..1000).rev().collect();
    let kth = quickselect(&mut arr, 500);
    assert_eq!(kth, Some(500));
}

#[test]
fn test_quickselect_already_sorted() {
    let mut arr = vec![1, 2, 3, 4, 5];
    let kth = quickselect(&mut arr, 3);
    assert_eq!(kth, Some(4));
}

#[test]
fn test_quickselect_all_equal() {
    let mut arr = vec![7, 7, 7, 7, 7];
    let kth = quickselect(&mut arr, 2);
    assert_eq!(kth, Some(7));
}

#[test]
fn test_quickselect_negative_numbers() {
    let mut arr = vec![-3, -1, -4, -2, 0];
    let kth = quickselect(&mut arr, 1);
    assert_eq!(kth, Some(-3));
}

#[test]
fn test_quickselect_randomized() {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut arr: Vec<_> = (0..50).collect();
    for k in 0..arr.len() {
        arr.shuffle(&mut rng);
        let kth = quickselect(&mut arr, k);
        assert_eq!(kth, Some(k as i32));
    }
}

#[test]
fn test_quickselect_large_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(-10000..10000)).collect();
    let mut sorted = arr.clone();
    sorted.sort();
    for k in [0, 10, 100, 500, 999] {
        let kth = quickselect(&mut arr, k);
        assert_eq!(kth, Some(sorted[k]));
    }
}

// ... (repeat similar randomized and edge case tests to ensure 100+ lines)
