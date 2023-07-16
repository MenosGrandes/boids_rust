use crate::logic::boid::boid_impl::Boid;
use crate::{
    constants::{
        BehaviourEnabled, ALLIGN_FACTOR, BEHAVIOUR_ENABLED, COHESION_FACTOR, MAX_BOID_SPEED,
        SEPERATE_FACTOR,
    },
    math::vec::{Magnitude, V2f32},
};

pub enum BorderBehaviourE {
    GoThrough,
    Reflect,
}

pub trait BorderBehaviour {
    fn border(&mut self, e: &BorderBehaviourE);
}
pub trait Behaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32;
}

pub struct AlignBehaviour;
impl Behaviour for AlignBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::ALLIGN) } {
            return V2f32::zero();
        }
        if other_boids.len() == 1 {
            return V2f32::zero();
        }
        //There is only same ID boid in array.
        let mut avarage_velocity: V2f32 = other_boids
            .iter()
            .filter(|b| b.id != self_boid.id)
            .map(|boid| boid.velocity)
            .sum::<V2f32>()
            / (other_boids.len() as f32);

        avarage_velocity.set_magnitude(MAX_BOID_SPEED);
        avarage_velocity -= self_boid.velocity;
        avarage_velocity *= ALLIGN_FACTOR;
        return avarage_velocity;
    }
}
pub struct CohesionBehaviour;
impl Behaviour for CohesionBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::COHESION) } {
            return V2f32::zero();
        }
        if other_boids.len() == 1 {
            return V2f32::zero();
        }
        let mut avarage_position: V2f32 = other_boids
            .iter()
            .filter(|b| b.id != self_boid.id)
            .map(|boid| boid.position)
            .sum::<V2f32>()
            / (other_boids.len() as f32);

        avarage_position -= self_boid.position;
        avarage_position.set_magnitude(MAX_BOID_SPEED);
        avarage_position -= self_boid.velocity;
        avarage_position *= COHESION_FACTOR;

        return avarage_position;
    }
}
pub struct SeperateBehaviour;
impl Behaviour for SeperateBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::SEPERATE) } {
            return V2f32::zero();
        }
        if other_boids.len() == 1 {
            return V2f32::zero();
        }
        let mut avarage_position: V2f32 = other_boids
            .iter()
            .filter(|b| b.id != self_boid.id)
            .map(|boid| self_boid.position - boid.position)
            .sum::<V2f32>()
            / (other_boids.len() as f32);

        avarage_position.set_magnitude(MAX_BOID_SPEED);
        avarage_position -= self_boid.velocity;
        avarage_position *= SEPERATE_FACTOR;

        return avarage_position;
    }
}
pub trait SeeBehaviour {
    fn get_other_visible(self, other: &[Boid]) -> Vec<Boid>;
}
