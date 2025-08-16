//! Comb Sort Example
//!
//! Demonstrates usage of the generic, production-grade comb_sort algorithm.

use lunaris_engine::list::comb_sort::comb_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    comb_sort(&mut data);
    println!("Sorted: {:?}", data);
}
