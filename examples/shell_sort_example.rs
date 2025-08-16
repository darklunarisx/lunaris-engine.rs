//! Shell Sort Example
//!
//! Demonstrates usage of the generic, production-grade shell_sort algorithm.

use lunaris_engine::list::shell_sort::shell_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    shell_sort(&mut data);
    println!("Sorted: {:?}", data);
}
