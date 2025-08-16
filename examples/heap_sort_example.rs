//! Heap Sort Example
//!
//! Demonstrates usage of the generic, production-grade heap_sort algorithm.

use lunaris_engine::list::heap_sort::heap_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    heap_sort(&mut data);
    println!("Sorted: {:?}", data);
}
