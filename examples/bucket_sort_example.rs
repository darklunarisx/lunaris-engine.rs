//! Bucket Sort Example
//!
//! Demonstrates usage of the generic, production-grade bucket_sort algorithm.

use lunaris_engine::list::bucket_sort::bucket_sort;

fn main() {
    let mut data = vec![0.42, 4.21, 3.14, 2.71, 1.61];
    bucket_sort(&mut data);
    println!("Sorted: {:?}", data);
}
