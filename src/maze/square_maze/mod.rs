mod dfs;

use super::MazeAlgo;
use bit_vec::BitVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SquareMaze {
    width: usize,
    height: usize,
    walls: BitVec,
    algo: MazeAlgo,
    seed: u64,
}

#[wasm_bindgen]
impl SquareMaze {
    /// Initialises a SquareMaze with a specified algorithm and a 64-bit seed,
    /// provided via 2 32-bit values.
    pub fn from_seed(
        width: usize,
        height: usize,
        algo: MazeAlgo,
        seed_p1: u32,
        seed_p2: u32,
    ) -> SquareMaze {
        let walls = BitVec::from_elem(width * (height - 1) * (width - 1) * height, false);
        let seed = (seed_p1 as u64) & ((seed_p2 as u64) << 32);

        let mut mz = SquareMaze {
            width,
            height,
            walls,
            algo,
            seed,
        };
        match algo {
            MazeAlgo::Dfs => mz.init_dfs(),
        }

        mz
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn seed_p1(&self) -> u32 {
        (self.seed & 0xFFFFFFFF) as u32
    }
    pub fn seed_p2(&self) -> u32 {
        (self.seed >> 32) as u32
    }
}
