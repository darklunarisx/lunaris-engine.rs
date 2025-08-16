//! Quickselect Example
//!
//! Demonstrates usage of the generic, production-grade quickselect algorithm.

use lunaris_engine::list::quickselect::quickselect;

fn main() {
    let mut data = vec![7, 10, 4, 3, 20, 15];
    let kth = quickselect(&mut data, 2);
    println!("2nd smallest: {:?}", kth);
}
