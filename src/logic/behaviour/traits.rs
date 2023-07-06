use crate::{logic::boid::Boid, math::vec::V2f32};

pub enum BorderBehaviourE {
    GoThrough,
    Reflect,
}

pub trait BorderBehaviour {
    fn border(&mut self, e: &BorderBehaviourE);
}
pub trait FlockBehaviour {
    fn align(&mut self, other: &[Boid]) -> V2f32;
    fn cohesion(&mut self, other: &[Boid]) -> V2f32;
    fn seperate(&mut self, other: &[Boid]) -> V2f32;
}
pub trait SeeBehaviour {
    fn get_other_visible(self, other: &[Boid]) -> Vec<Boid>;
}
