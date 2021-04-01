extern crate alloc;

use super::{SqDir, SquareMaze};
use alloc::vec::Vec;
use bit_vec::BitVec;
use rand::prelude::*;
use rand::rngs::SmallRng;

impl SquareMaze {
    pub fn init_dfs(&mut self) {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut stk = Vec::new();

        let mut visited = BitVec::from_elem(self.width * self.height, false);
        let init_pos = rng.gen_range(0..self.width * self.height);
        let mut dir_buf = Vec::with_capacity(4);

        stk.push((init_pos, None));
        while !stk.is_empty() {
            let (at, dir) = stk.pop().unwrap();
            if !visited[at] {
                visited.set(at, true);
                if let Some(dir) = dir {
                    self.walls.set(self.get_wall(at, dir).unwrap(), true);
                }
                let y = at / self.width;
                let x = at % self.width;

                dir_buf.clear();
                if y != 0 {
                    dir_buf.push((at - self.width, Some(SqDir::Down)));
                }
                if x != 0 {
                    dir_buf.push((at - 1, Some(SqDir::Right)));
                }
                if y != self.height - 1 {
                    dir_buf.push((at + self.width, Some(SqDir::Up)));
                }
                if x != self.width - 1 {
                    dir_buf.push((at + 1, Some(SqDir::Left)));
                }
                dir_buf.shuffle(&mut rng);
                for neighbor in dir_buf.iter() {
                    stk.push(*neighbor);
                }
            }
        }
    }
}
