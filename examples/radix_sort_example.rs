//! Radix Sort Example
//!
//! Demonstrates usage of the generic, production-grade radix_sort algorithm.

use lunaris_engine::list::radix_sort::radix_sort;

fn main() {
    let mut data: Vec<u64> = vec![170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort(&mut data);
    println!("Sorted: {:?}", data);
}
