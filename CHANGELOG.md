# Total Algorithms: 121

# v0.1.0 (2025-08-11)

## Major Additions

**Total algorithms added: 23**

### list Algorithms (16)

### Set Algorithms (7)

## Other Changes
- Added robust, dedicated test files for each new algorithm in tree, linked list, and DP modules.
- Added comprehensive documentation and doctests for all new algorithms.
- Created full example files: `examples/tree_algorithms_examples.rs`, `examples/linked_list_examples.rs`, `examples/dp_algorithms_examples.rs` demonstrating usage of all new algorithms.

# v0.2.0 (2025-08-11)

## [Unreleased]

### Major Additions (August 16, 2025)

**Total advanced list algorithms added: 13**

#### Advanced List Algorithms
- Heap Sort (`list::heap_sort`)
- Shell Sort (`list::shell_sort`)
- Gnome Sort (`list::gnome_sort`)
- Odd-Even Sort (`list::odd_even_sort`)
- Pancake Sort (`list::pancake_sort`)
- Cycle Sort (`list::cycle_sort`)
- Bucket Sort (`list::bucket_sort`)
- Radix Sort (`list::radix_sort`)
- Pigeonhole Sort (`list::pigeonhole_sort`)
- Bitonic Sort (`list::bitonic_sort`)
- Comb Sort (`list::comb_sort`)
- Stooge Sort (`list::stooge_sort`)
- Quickselect (k-th smallest) (`list::quickselect`)
- LIS using Binary Search (O(n log n)) (`list::lis_binary_search`)
- Maximum Product Subarray (`list::maximum_product_subarray`)

#### Other Changes
- Added robust, dedicated test files for each new advanced list algorithm (each >100 lines).
- Added comprehensive documentation and doctests for all new algorithms.
- Created full example files for each new algorithm in `examples/`.
- Added `examples/advanced_list_algorithms_exam.rs` demonstrating and validating all new advanced list algorithms.

### Major Additions

**Total graph algorithms added: 10+**

### Graph Algorithms (10+)
- Kruskal's Minimum Spanning Tree (MST) algorithm (`graph::kruskal`)
- Prim's Minimum Spanning Tree (MST) algorithm (`graph::prim`)
- Kosaraju's Strongly Connected Components (SCC) algorithm (`graph::kosaraju_scc`)
- Articulation Points (Cut Vertices) algorithm (`graph::articulation_points`)

## Other Changes
- Added robust, dedicated test files for each new graph algorithm.
- Comprehensive documentation and doctests for all new graph algorithms.

## Major Additions

**Total string algorithms added: 10**

### String Algorithms (10)
- Reverse String
- Palindrome Check
- Anagram Check (for strings)
- Longest Palindromic Substring
- String Compression
- Substring Search (Brute Force)
- Rabin-Karp
- Longest Common Prefix
- Edit Distance
- Count Vowels/Consonants

## Tree Algorithms (11)
- Binary Tree Traversals: Inorder, Preorder, Postorder (`tree::binary_tree_traversal`)
- Diameter of Binary Tree (`tree::diameter_of_tree`)
- Balanced Binary Tree Check (`tree::balanced_tree`)
- Level Order (BFS) Traversal (`tree::level_order_traversal`)
- Lowest Common Ancestor (LCA) (`tree::lowest_common_ancestor`)
- Invert Binary Tree (`tree::invert_tree`)
- Zigzag Level Order Traversal (`tree::zigzag_traversal`)
- Validate Binary Search Tree (`tree::validate_bst`)
- Depth of Binary Tree (`tree::tree_depth`)
- Serialize and Deserialize Binary Tree (`tree::serialize_deserialize`)
- Robust, dedicated test files and a full example file for all tree algorithms

## Linked list Algorithms (9)
- Singly Linked list Node and Traversal (`linked_list::singly_linked_list`)
- Insert and Delete at Position (`linked_list::insert_delete`)
- Doubly Linked list Node and Reverse (`linked_list::doubly_linked_list`)
- Reverse Singly Linked list (`linked_list::reverse_list`)
- Detect Cycle (Floyd's) (`linked_list::detect_cycle`)
- Merge Two Sorted lists (`linked_list::merge_sorted`)
- Remove Nth Node From End (`linked_list::remove_nth_from_end`)
- Palindrome Linked list Check (`linked_list::palindrome`)
- Intersection of Two lists (`linked_list::intersection`)
- Robust, dedicated test files and a full example file for all linked list algorithms

## Dynamic Programming Algorithms (13)
- Fibonacci (Memoization) (`dp_algorithms::fibonacci`)
- 0/1 Knapsack (`dp_algorithms::knapsack_01`)
- Longest Increasing Subsequence (`dp_algorithms::longest_increasing_subsequence`)
- Longest Common Subsequence (`dp_algorithms::longest_common_subsequence`)
- Edit Distance (`dp_algorithms::edit_distance`)
- Matrix Path Sum (`dp_algorithms::matrix_path_sum`)
- Coin Change (`dp_algorithms::coin_change`)
- Subset Sum (`dp_algorithms::subset_sum`)
- Partition Equal Subset Sum (`dp_algorithms::partition_equal_subset_sum`)
- House Robber (`dp_algorithms::house_robber`)
- Jump Game (`dp_algorithms::jump_game`)
- Palindromic Substrings (`dp_algorithms::palindromic_substrings`)
- Rod Cutting (`dp_algorithms::rod_cutting`)
- Robust, dedicated test files and a full example file for all DP algorithms

## Backtracking Algorithms (9)

## Matrix/Grid Algorithms (8)
- Flood Fill (`matrix::flood_fill`)
- Island Count (DFS/BFS) (`matrix::island_count`)
- Shortest Path in Grid (`matrix::shortest_path_grid`)
- Word Search (`matrix::word_search`)
- Path Sum in Matrix (`matrix::path_sum`)
- Matrix Rotation (`matrix::matrix_rotation`)
- Spiral Traversal (`matrix::spiral_traversal`)
- Surrounded Regions (`matrix::surrounded_regions`)
- Robust, dedicated test files and a full example file for all matrix/grid algorithms