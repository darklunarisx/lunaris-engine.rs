//! Corporate-grade, generic, and highly optimized list algorithms for Rust.
//!
//! # Overview
//! This module provides a suite of high-performance, production-ready algorithms for searching and sorting lists, as well as common list operations. All algorithms are generic over type `T` and leverage Rust's trait system for maximum flexibility and safety.
//!
//! ## Algorithms Included
//! - Linear Search
//! - Binary Search
//! - Bubble Sort
//! - Selection Sort
//! - Insertion Sort
//! - Merge Sort
//! - Quick Sort
//! - Counting Sort
//! - Reverse list
//! - Find Max/Min
//! - Find Duplicates
//! - Kadane's Algorithm (Maximum Subarray Sum)
//! - Sliding Window
//! - Two Pointers
//! - Remove Duplicates
//! - Rotate Array
//! - Prefix Sum
//!
//! # Usage
//! ```rust
//! use lunaris_engine::list::linear_search::linear_search;
//! let idx = linear_search(&[1, 2, 3], &2);
//! ```

pub mod binary_search;
pub mod bitonic_sort;
pub mod bubble_sort;
pub mod bucket_sort;
pub mod comb_sort;
pub mod counting_sort;
pub mod cycle_sort;
pub mod find_duplicates;
pub mod find_max_min;
pub mod gnome_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod kadanes_algorithm;
pub mod linear_search;
pub mod lis_binary_search;
pub mod maximum_product_subarray;
pub mod merge_sort;
pub mod odd_even_sort;
pub mod pancake_sort;
pub mod pigeonhole_sort;
pub mod prefix_sum;
pub mod quick_sort;
pub mod quickselect;
pub mod radix_sort;
pub mod remove_duplicates;
pub mod reverse_list;
pub mod rotate_array_right;
pub mod selection_sort;
pub mod shell_sort;
pub mod sliding_window;
pub mod stooge_sort;
