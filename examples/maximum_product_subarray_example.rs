//! Maximum Product Subarray Example
//!
//! Demonstrates usage of the generic, production-grade maximum_product_subarray algorithm.

use lunaris_engine::list::maximum_product_subarray::maximum_product_subarray;

fn main() {
    let arr = vec![2, 3, -2, 4];
    let max_prod = maximum_product_subarray(&arr);
    println!("Maximum product: {}", max_prod);
}
