//! LIS (Binary Search) Example
//!
//! Demonstrates usage of the generic, production-grade lis_length algorithm.

use lunaris_engine::list::lis_binary_search::lis_length;

fn main() {
    let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let len = lis_length(&arr);
    println!("LIS length: {}", len);
}
