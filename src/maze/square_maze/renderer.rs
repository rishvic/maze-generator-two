extern crate alloc;

use super::SquareMaze;
use alloc::vec::Vec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SqMzBuffer {
    r: f32,
    verts: Vec<f32>,
    inds: Vec<u32>,
}

#[wasm_bindgen]
impl SqMzBuffer {
    pub fn new(mz: &SquareMaze, r: f32) -> SqMzBuffer {
        let w = mz.width();
        let h = mz.height();
        let verts = Vec::with_capacity((w * h + 3 - w - h) << 4);
        let inds = Vec::with_capacity(12 * (w * h + 3 - w - h));

        let tw = 1.0 / ((w + 1) as f32 * (1.0 + r) - 1.0);
        let th = 1.0 / ((h + 1) as f32 * (1.0 + r) - 1.0);

        let mut mz_buf = SqMzBuffer { r, verts, inds };

        let mzw = (w << 1) - 1;
        for i in 0..h - 1 {
            let py = (i + 1) as f32 * (1.0 + r) * th;
            for j in 0..w {
                let px = r * tw + j as f32 * (1.0 + r) * tw;

                let y = (i << 1) | 1;
                let x = j << 1;
                let cellno = ((y * mzw) + x) >> 1;

                if !mz[cellno] {
                    mz_buf.push_rect(px, py, px + tw, py + r * th);
                }
            }
        }

        for i in 0..h {
            let py = r * th + i as f32 * (1.0 + r) * th;
            for j in 0..w - 1 {
                let px = (j + 1) as f32 * (1.0 + r) * tw;

                let y = i << 1;
                let x = (j << 1) | 1;
                let cellno = ((y * mzw) + x) >> 1;

                if !mz[cellno] {
                    mz_buf.push_rect(px, py, px + r * tw, py + th);
                }
            }
        }

        for i in 0..h - 1 {
            let py = (i + 1) as f32 * (1.0 + r) * th;
            for j in 0..w - 1 {
                let px = (j + 1) as f32 * (1.0 + r) * tw;
                mz_buf.push_rect(px, py, px + r * tw, py + r * th);
            }
        }

        mz_buf.push_rect((1.0 + r) * tw, 0.0, 1.0, r * th);
        mz_buf.push_rect(1.0 - r * tw, r * th, 1.0, 1.0);
        mz_buf.push_rect(0.0, 1.0 - r * th, 1.0 - (1.0 + r) * tw, 1.0);
        mz_buf.push_rect(0.0, 0.0, r * tw, 1.0 - r * th);

        mz_buf
    }

    pub fn get_vertices(&self) -> *const f32 {
        self.verts.as_ptr()
    }

    pub fn get_vertices_count(&self) -> usize {
        self.verts.len()
    }

    pub fn get_indices(&self) -> *const u32 {
        self.inds.as_ptr()
    }

    pub fn get_indices_count(&self) -> usize {
        self.inds.len()
    }
}

impl SqMzBuffer {
    fn push_rect(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let x1 = 2.0 * x1 - 1.0;
        let x2 = 2.0 * x2 - 1.0;
        let y1 = 2.0 * y1 - 1.0;
        let y2 = 2.0 * y2 - 1.0;

        let sz = self.verts.len() >> 1;
        self.verts.push(x1);
        self.verts.push(y1);
        self.verts.push(x2);
        self.verts.push(y1);
        self.verts.push(x1);
        self.verts.push(y2);
        self.verts.push(x2);
        self.verts.push(y2);

        self.inds.push(sz as u32);
        self.inds.push((sz + 1) as u32);
        self.inds.push((sz + 2) as u32);
        self.inds.push((sz + 3) as u32);
        self.inds.push((sz + 2) as u32);
        self.inds.push((sz + 1) as u32);
    }
}
