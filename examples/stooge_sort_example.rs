//! Stooge Sort Example
//!
//! Demonstrates usage of the generic, production-grade stooge_sort algorithm.

use lunaris_engine::list::stooge_sort::stooge_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    stooge_sort(&mut data);
    println!("Sorted: {:?}", data);
}
