use crate::constants::{
    BehaviourConsts, BehaviourEnabled, BEHAVIOUR_ENABLED, MAX_BOID_SPEED, SCREEN_SIZE,
};
use crate::logic::boid::boid_impl::Boid;
use crate::math::quadtree::region::Region;
use crate::math::vec::{Magnitude, V2f32};
use crate::math::vec::{Normalize, Vector2};

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
        let mut avarage_velocity: V2f32 = other_boids
            .iter()
            .map(|boid| boid.velocity.calc_normalize())
            .sum::<V2f32>()
            / (other_boids.len() as f32);

        avarage_velocity.set_magnitude(MAX_BOID_SPEED);
        avarage_velocity -= self_boid.velocity;
        avarage_velocity *= BehaviourConsts::ALLIGN_FACTOR;
        return avarage_velocity;
    }
}
pub struct CohesionBehaviour;
impl Behaviour for CohesionBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::COHESION) } {
            return V2f32::zero();
        }
        let mut avarage_position: V2f32 =
            other_boids.iter().map(|boid| boid.position).sum::<V2f32>()
                / (other_boids.len() as f32);

        avarage_position -= self_boid.position;
        avarage_position.set_magnitude(MAX_BOID_SPEED);
        avarage_position -= self_boid.velocity;
        avarage_position *= BehaviourConsts::COHESION_FACTOR;

        return avarage_position;
    }
}
pub struct SeperateBehaviour;
impl Behaviour for SeperateBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::SEPERATE) } {
            return V2f32::zero();
        }
        let mut avarage_position: V2f32 = other_boids
            .iter()
            .map(|boid| self_boid.position - boid.position)
            .sum::<V2f32>()
            / (other_boids.len() as f32);

        avarage_position.set_magnitude(MAX_BOID_SPEED);
        avarage_position -= self_boid.velocity;
        avarage_position *= BehaviourConsts::SEPERATE_FACTOR;

        return avarage_position;
    }
}

pub struct BoundBehaviour;
impl Behaviour for BoundBehaviour {
    fn calculate(&self, self_boid: &Boid, _other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::BOUND) } {
            return V2f32::zero();
        }

        let r: Region = Region::new(
            Vector2::new(100.0, 100.0),
            Vector2::new((SCREEN_SIZE.x - 100) as f32, (SCREEN_SIZE.y - 100) as f32),
        );
        let turnfactor = 0.25;
        let x = if self_boid.position.x < r.left_up.x {
            turnfactor
        } else if self_boid.position.x > r.right_down.x {
            -turnfactor
        } else {
            0.0
        };

        let y = if self_boid.position.y < r.left_up.y {
            turnfactor
        } else if self_boid.position.y > r.right_down.y {
            -turnfactor
        } else {
            0.0
        };
        Vector2::new(x, y)
    }
}
