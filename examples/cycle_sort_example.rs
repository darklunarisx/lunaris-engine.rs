//! Cycle Sort Example
//!
//! Demonstrates usage of the generic, production-grade cycle_sort algorithm.

use lunaris_engine::list::cycle_sort::cycle_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    cycle_sort(&mut data);
    println!("Sorted: {:?}", data);
}
