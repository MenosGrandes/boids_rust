use crate::math::vec::V2f32;

pub trait Updatable {
    fn update(&mut self);
}
pub trait UpdatableAcceleration {
    fn update(&mut self, acceleration: V2f32);
}
