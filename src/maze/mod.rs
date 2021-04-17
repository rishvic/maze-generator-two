pub mod square_maze;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum MazeAlgo {
    Dfs,
    Kruskal,
    Wilson,
}
