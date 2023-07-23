use crate::math::{quadtree::region::Region, vec::V2f32};

#[derive(Debug)]
pub struct Camera {
    pub pos: V2f32,
}
impl Camera {
    pub fn new(pos: V2f32) -> Self {
        Self { pos }
    }
    pub fn calc_pos_v2f32(&self, pos: V2f32) -> V2f32 {
        pos - self.pos
    }

    // add code here
}
