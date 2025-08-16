//! Exam file for all advanced list algorithms
//!
//! This file demonstrates and validates the usage of all advanced, production-grade list algorithms in the library.

use lunaris_engine::list::*;

fn main() {
    // Heap Sort
    let mut heap = vec![5, 3, 8, 4, 2];
    heap_sort::heap_sort(&mut heap);
    println!("Heap Sort: {:?}", heap);

    // Shell Sort
    let mut shell = vec![5, 3, 8, 4, 2];
    shell_sort::shell_sort(&mut shell);
    println!("Shell Sort: {:?}", shell);

    // Gnome Sort
    let mut gnome = vec![5, 3, 8, 4, 2];
    gnome_sort::gnome_sort(&mut gnome);
    println!("Gnome Sort: {:?}", gnome);

    // Odd-Even Sort
    let mut odd_even = vec![5, 3, 8, 4, 2];
    odd_even_sort::odd_even_sort(&mut odd_even);
    println!("Odd-Even Sort: {:?}", odd_even);

    // Pancake Sort
    let mut pancake = vec![5, 3, 8, 4, 2];
    pancake_sort::pancake_sort(&mut pancake);
    println!("Pancake Sort: {:?}", pancake);

    // Cycle Sort
    let mut cycle = vec![5, 3, 8, 4, 2];
    cycle_sort::cycle_sort(&mut cycle);
    println!("Cycle Sort: {:?}", cycle);

    // Bucket Sort
    let mut bucket = vec![0.42, 4.21, 3.14, 2.71, 1.61];
    bucket_sort::bucket_sort(&mut bucket);
    println!("Bucket Sort: {:?}", bucket);

    // Radix Sort
    let mut radix = vec![170u32, 45u32, 75u32, 90u32, 802u32, 24u32, 2u32, 66u32];
    radix_sort::radix_sort(&mut radix);
    println!("Radix Sort: {:?}", radix);

    // Pigeonhole Sort
    let mut pigeonhole = vec![8, 3, 2, 7, 4, 6, 8, 5];
    pigeonhole_sort::pigeonhole_sort(&mut pigeonhole);
    println!("Pigeonhole Sort: {:?}", pigeonhole);

    // Bitonic Sort
    let mut bitonic = vec![3, 7, 4, 8, 6, 2, 1, 5];
    bitonic_sort::bitonic_sort(&mut bitonic, true);
    println!("Bitonic Sort: {:?}", bitonic);

    // Comb Sort
    let mut comb = vec![5, 3, 8, 4, 2];
    comb_sort::comb_sort(&mut comb);
    println!("Comb Sort: {:?}", comb);

    // Stooge Sort
    let mut stooge = vec![5, 3, 8, 4, 2];
    stooge_sort::stooge_sort(&mut stooge);
    println!("Stooge Sort: {:?}", stooge);

    // Quickselect
    let mut quickselect = vec![7, 10, 4, 3, 20, 15];
    let kth = quickselect::quickselect(&mut quickselect, 2);
    println!("Quickselect (2nd smallest): {:?}", kth);

    // LIS (Binary Search)
    let lis = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let lis_len = lis_binary_search::lis_length(&lis);
    println!("LIS length: {}", lis_len);

    // Maximum Product Subarray
    let max_prod = maximum_product_subarray::maximum_product_subarray(&[2, 3, -2, 4]);
    println!("Maximum Product Subarray: {}", max_prod);
}
