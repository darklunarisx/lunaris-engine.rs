//! Odd-Even Sort Example
//!
//! Demonstrates usage of the generic, production-grade odd_even_sort algorithm.

use lunaris_engine::list::odd_even_sort::odd_even_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    odd_even_sort(&mut data);
    println!("Sorted: {:?}", data);
}
