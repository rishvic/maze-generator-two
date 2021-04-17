extern crate alloc;

mod dfs;
mod kruskal;
pub mod renderer;
mod wilson;

use super::MazeAlgo;
use crate::utils::set_panic_hook;
use alloc::vec::Vec;
use bit_vec::BitVec;
use core::ops::Index;
use wasm_bindgen::prelude::*;

/// Struct representing a square maze. Indexing goes from left-to-right, then
/// top-to-bottom. `false` in the `walls` array represents an unpassable wall,
/// and `true` represents a passable wall.
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
        set_panic_hook();
        let walls = BitVec::from_elem(width * (height - 1) + (width - 1) * height, false);
        let seed = (seed_p1 as u64) | ((seed_p2 as u64) << 32);

        let mut mz = SquareMaze {
            width,
            height,
            walls,
            algo,
            seed,
        };
        match algo {
            MazeAlgo::Dfs => mz.init_dfs(),
            MazeAlgo::Kruskal => mz.init_kruskal(),
            MazeAlgo::Wilson => mz.init_wilson(),
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

#[derive(Debug, Clone, Copy)]
enum SqDir {
    Up,
    Left,
    Down,
    Right,
}

impl SquareMaze {
    fn get_wall(&self, cell: usize, w_dir: SqDir) -> Option<usize> {
        let y = cell / self.width;
        let x = cell % self.width;
        match w_dir {
            SqDir::Up => {
                if y == 0 {
                    None
                } else {
                    Some(x + (y - 1) * ((self.width << 1) - 1) + self.width - 1)
                }
            }
            SqDir::Left => {
                if x == 0 {
                    None
                } else {
                    Some(x - 1 + y * ((self.width << 1) - 1))
                }
            }
            SqDir::Down => {
                if y == self.height - 1 {
                    None
                } else {
                    Some(x + y * ((self.width << 1) - 1) + self.width - 1)
                }
            }
            SqDir::Right => {
                if x == self.width - 1 {
                    None
                } else {
                    Some(x + y * ((self.width << 1) - 1))
                }
            }
        }
    }

    pub fn is_traversable_no_loops(&self) -> bool {
        let mut stk = Vec::new();

        let mut visited = BitVec::from_elem(self.width * self.height, false);
        let init_pos = 0;
        let mut dir_buf = Vec::with_capacity(4);

        stk.push((init_pos, usize::MAX));
        while !stk.is_empty() {
            let (at, p) = stk.pop().unwrap();

            if visited[at] {
                return false;
            }

            visited.set(at, true);
            let y = at / self.width;
            let x = at % self.width;

            dir_buf.clear();
            if y != 0 && at - self.width != p && self.walls[self.get_wall(at, SqDir::Up).unwrap()] {
                dir_buf.push((at - self.width, at));
            }
            if x != 0 && at - 1 != p && self.walls[self.get_wall(at, SqDir::Left).unwrap()] {
                dir_buf.push((at - 1, at));
            }
            if y != self.height - 1
                && at + self.width != p
                && self.walls[self.get_wall(at, SqDir::Down).unwrap()]
            {
                dir_buf.push((at + self.width, at));
            }
            if x != self.width - 1
                && at + 1 != p
                && self.walls[self.get_wall(at, SqDir::Right).unwrap()]
            {
                dir_buf.push((at + 1, at));
            }
            for neighbor in dir_buf.iter() {
                stk.push(*neighbor);
            }
        }

        return visited.all();
    }
}

impl Index<usize> for SquareMaze {
    type Output = bool;

    fn index(&self, ind: usize) -> &Self::Output {
        self.walls.index(ind)
    }
}
