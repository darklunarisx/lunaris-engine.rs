//! Bitonic Sort Example
//!
//! Demonstrates usage of the generic, production-grade bitonic_sort algorithm.

use lunaris_engine::list::bitonic_sort::bitonic_sort;

fn main() {
    let mut data = vec![3, 7, 4, 8, 6, 2, 1, 5];
    bitonic_sort(&mut data, true);
    println!("Sorted: {:?}", data);
}
