//! Test suite for check algorithms

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use maze_generator_two::maze::{square_maze::SquareMaze, MazeAlgo};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_valid_dfs_maze() {
    let mz = SquareMaze::from_seed(731, 801, MazeAlgo::Dfs, 3414285684, 2205782953);
    assert!(mz.is_traversable_no_loops());
}

#[wasm_bindgen_test]
fn test_valid_kruskal_maze() {
    let mz = SquareMaze::from_seed(692, 735, MazeAlgo::Kruskal, 2531773449, 1724692826);
    assert!(mz.is_traversable_no_loops());
}

#[wasm_bindgen_test]
fn test_valid_wilson_maze() {
    let mz = SquareMaze::from_seed(922, 856, MazeAlgo::Wilson, 3013860346, 935133577);
    assert!(mz.is_traversable_no_loops());
}
