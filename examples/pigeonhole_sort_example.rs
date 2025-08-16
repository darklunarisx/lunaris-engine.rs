//! Pigeonhole Sort Example
//!
//! Demonstrates usage of the generic, production-grade pigeonhole_sort algorithm.

use lunaris_engine::list::pigeonhole_sort::pigeonhole_sort;

fn main() {
    let mut data = vec![8, 3, 2, 7, 4, 6, 8, 5];
    pigeonhole_sort(&mut data);
    println!("Sorted: {:?}", data);
}
