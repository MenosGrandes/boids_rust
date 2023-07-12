use crate::{logic::boid::Boid, math::vec::V2f32};

pub enum BorderBehaviourE {
    GoThrough,
    Reflect,
}

pub trait BorderBehaviour {
    fn border(&mut self, e: &BorderBehaviourE);
}
pub trait FlockBehaviour {
    fn align(&self, other: &[Boid]) -> V2f32;
    fn cohesion(&self, other: &[Boid]) -> V2f32;
    fn seperate(&self, other: &[Boid]) -> V2f32;
}
pub trait SeeBehaviour {
    fn get_other_visible(self, other: &[Boid]) -> Vec<Boid>;
}
