//! Gnome Sort Example
//!
//! Demonstrates usage of the generic, production-grade gnome_sort algorithm.

use lunaris_engine::list::gnome_sort::gnome_sort;

fn main() {
    let mut data = vec![9, 4, 7, 1, 3, 6, 2, 8, 5];
    gnome_sort(&mut data);
    println!("Sorted: {:?}", data);
}
