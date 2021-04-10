extern crate alloc;

use super::SquareMaze;
use alloc::vec::Vec;
use rand::prelude::*;
use rand::rngs::SmallRng;

struct UnionFind {
    p: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let p = (0..n).collect();
        let mut rank = Vec::with_capacity(n);
        rank.resize(n, 0);
        UnionFind { p, rank }
    }

    fn find_set(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            i
        } else {
            self.p[i] = self.find_set(self.p[i]);
            self.p[i]
        }
    }

    fn is_same_set(&mut self, i: usize, j: usize) -> bool {
        self.find_set(i) == self.find_set(j)
    }

    fn union_set(&mut self, i: usize, j: usize) {
        if !self.is_same_set(i, j) {
            let i = self.find_set(i);
            let j = self.find_set(j);
            if self.rank[i] > self.rank[j] {
                self.p[j] = i;
            } else {
                self.p[i] = j;
                self.rank[j] += (self.rank[i] == self.rank[j]) as usize;
            }
        }
    }
}

impl SquareMaze {
    pub fn init_kruskal(&mut self) {
        let w = self.width;
        let h = self.height;
        let mzw = (w << 1) - 1;

        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut udfs = UnionFind::new(w * h);
        let mut walls: Vec<usize> = (0..w * (h - 1) + (w - 1) * h).collect();
        walls.shuffle(&mut rng);

        let mut comps = w * h;
        let mut c1;
        let mut c2;
        for wall in walls {
            let mzwall = (wall << 1) | 1;
            match (mzwall / mzw) & 1 {
                0 => {
                    c1 = (((mzwall - 1) % mzw) >> 1) + (((mzwall - 1) / mzw) >> 1) * w;
                    c2 = c1 + 1;
                }
                1 => {
                    c1 = (((mzwall - mzw) % mzw) >> 1) + (((mzwall - mzw) / mzw) >> 1) * w;
                    c2 = c1 + w;
                }
                _ => panic!("{}", "{integer} & 1 should only return 0 or 1"),
            }

            if !udfs.is_same_set(c1, c2) {
                self.walls.set(wall, true);
                comps -= 1;
                if comps == 1 {
                    break;
                }
                udfs.union_set(c1, c2);
            }
        }
    }
}
