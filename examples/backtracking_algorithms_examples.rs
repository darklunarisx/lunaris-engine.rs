//! Examples for all backtracking_algorithms
use lunaris_engine::backtracking::n_queens::n_queens;
use lunaris_engine::backtracking::sudoku_solver::solve_sudoku;
use lunaris_engine::backtracking::subset_generation::subset_generation;
use lunaris_engine::backtracking::permutations::permutations;
use lunaris_engine::backtracking::word_search::word_search;
use lunaris_engine::backtracking::combinations::combinations;
use lunaris_engine::backtracking::combination_sum::combination_sum;
use lunaris_engine::backtracking::letter_combinations_phone_number::letter_combinations;
use lunaris_engine::backtracking::rat_in_a_maze::rat_in_a_maze;

fn main() {
    // N-Queens
    let solutions = n_queens(4usize);
    println!("N-Queens(4) solutions: {:?}", solutions);

    // Sudoku Solver
    let mut board = [
        ['5','3','.','.','7','.','.','.','.'],
        ['6','.','.','1','9','5','.','.','.'],
        ['.','9','8','.','.','.','.','6','.'],
        ['8','.','.','.','6','.','.','.','3'],
        ['4','.','.','8','.','3','.','.','1'],
        ['7','.','.','.','2','.','.','.','6'],
        ['.','6','.','.','.','.','2','8','.'],
        ['.','.','.','4','1','9','.','.','5'],
        ['.','.','.','.','8','.','.','7','9']
    ];
    solve_sudoku(&mut board);
    println!("Sudoku solved: {:?}", board);

    // Subset Generation
    let nums = vec![1, 2, 3];
    let subsets = subset_generation(&nums);
    println!("Subsets: {:?}", subsets);

    // Permutations
    let perms = permutations(&nums);
    println!("Permutations: {:?}", perms);

    // Word Search
    let board = vec![vec!['A','B','C','E'], vec!['S','F','C','S'], vec!['A','D','E','E']];
    println!("Word Search (ABCCED): {}", word_search(&board, "ABCCED"));

    // Combinations
    let combs = combinations(&nums, 2);
    println!("Combinations of 2: {:?}", combs);

    // Combination Sum
    let candidates = vec![2,3,6,7];
    let target = 7;
    let result = combination_sum(&candidates, target);
    println!("Combination Sum: {:?}", result);

    // Letter Combinations of Phone Number
    let digits = "23";
    let result = letter_combinations(digits);
    println!("Letter Combinations: {:?}", result);

    // Rat in a Maze
    let maze = vec![vec![1,0,0,0], vec![1,1,0,1], vec![0,1,0,0], vec![1,1,1,1]];
    let paths = rat_in_a_maze(&maze);
    println!("Rat in a Maze paths: {:?}", paths);
}
