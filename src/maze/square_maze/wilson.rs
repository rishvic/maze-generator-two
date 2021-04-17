extern crate alloc;

use super::{SqDir, SquareMaze};
use alloc::vec::Vec;
use bit_vec::BitVec;
use rand::prelude::*;
use rand::{distributions::Uniform, rngs::SmallRng};

impl SquareMaze {
    pub fn init_wilson(&mut self) {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut ccount = self.width * self.height;
        let mut partof_mz = BitVec::from_elem(ccount, false);

        let cellgen = Uniform::from(0..ccount);

        let mut start_num = cellgen.sample(&mut rng);
        let mut cur_cell;
        let mut choice;
        let mut dir_buf = Vec::with_capacity(4);
        let mut dir_togo = Vec::with_capacity(ccount);
        dir_togo.resize(ccount, SqDir::Up);
        partof_mz.set(start_num, true);
        ccount -= 1;

        while ccount != 0 {
            start_num = cellgen.sample(&mut rng);

            cur_cell = start_num;
            while !partof_mz[cur_cell] {
                dir_buf.clear();
                if cur_cell / self.width != 0 {
                    dir_buf.push((cur_cell - self.width, SqDir::Up));
                }
                if cur_cell % self.width != 0 {
                    dir_buf.push((cur_cell - 1, SqDir::Left));
                }
                if cur_cell / self.width != self.height - 1 {
                    dir_buf.push((cur_cell + self.width, SqDir::Down));
                }
                if cur_cell % self.width != self.width - 1 {
                    dir_buf.push((cur_cell + 1, SqDir::Right));
                }
                choice = dir_buf.choose(&mut rng).unwrap();
                dir_togo[cur_cell] = choice.1;
                cur_cell = choice.0;
            }

            cur_cell = start_num;
            while !partof_mz[cur_cell] {
                partof_mz.set(cur_cell, true);
                ccount -= 1;
                self.walls
                    .set(self.get_wall(cur_cell, dir_togo[cur_cell]).unwrap(), true);
                match dir_togo[cur_cell] {
                    SqDir::Up => cur_cell -= self.width,
                    SqDir::Left => cur_cell -= 1,
                    SqDir::Down => cur_cell += self.width,
                    SqDir::Right => cur_cell += 1,
                }
            }
        }
    }
}
