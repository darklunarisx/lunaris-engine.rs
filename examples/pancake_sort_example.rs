//! Pancake Sort Example
//!
//! Demonstrates usage of the generic, production-grade pancake_sort algorithm.

use lunaris_engine::list::pancake_sort::pancake_sort;

fn main() {
	let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
	pancake_sort(&mut data);
	println!("Sorted: {:?}", data);
}
