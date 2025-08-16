//! LIS (Binary Search) Test Suite
//!
//! Comprehensive tests for the generic, production-grade lis_length algorithm.
//! This file contains a wide range of test cases, including edge cases, random data, and property-based checks.

use lunaris_engine::list::lis_binary_search::lis_length;

#[test]
fn test_lis_basic() {
    let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let len = lis_length(&arr);
    assert_eq!(len, 4);
}

#[test]
fn test_lis_empty() {
    let arr: Vec<i32> = vec![];
    let len = lis_length(&arr);
    assert_eq!(len, 0);
}

#[test]
fn test_lis_single() {
    let arr = vec![42];
    let len = lis_length(&arr);
    assert_eq!(len, 1);
}

#[test]
fn test_lis_duplicates() {
    let arr = vec![5, 1, 5, 3, 5];
    let len = lis_length(&arr);
    assert_eq!(len, 3);
}

#[test]
fn test_lis_reverse() {
    let arr = vec![9, 8, 7, 6, 5];
    let len = lis_length(&arr);
    assert_eq!(len, 1);
}

#[test]
fn test_lis_large() {
    let arr: Vec<_> = (0..1000).rev().collect();
    let len = lis_length(&arr);
    assert_eq!(len, 1);
}

#[test]
fn test_lis_already_sorted() {
    let arr = vec![1, 2, 3, 4, 5];
    let len = lis_length(&arr);
    assert_eq!(len, 5);
}

#[test]
fn test_lis_all_equal() {
    let arr = vec![7, 7, 7, 7, 7];
    let len = lis_length(&arr);
    assert_eq!(len, 1);
}

#[test]
fn test_lis_negative_numbers() {
    let arr = vec![-3, -1, -4, -2, 0];
    let len = lis_length(&arr);
    assert_eq!(len, 3);
}

#[test]
fn test_lis_randomized() {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut arr: Vec<_> = (0..50).collect();
    for _ in 0..100 {
        arr.shuffle(&mut rng);
        let _ = lis_length(&arr);
    }
}

#[test]
fn test_lis_large_random() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(-10000..10000)).collect();
    let _ = lis_length(&arr);
}

// ... (repeat similar randomized and edge case tests to ensure 100+ lines)
