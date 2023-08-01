use crate::constants::{
    BehaviourConsts, BehaviourEnabled, BEHAVIOUR_ENABLED, MAX_BOID_SPEED, SCREEN_SIZE,
    VIEW_PORT_SIZE,
};
use crate::logic::boid::boid_impl::Boid;
use crate::math::quadtree::region::Region;
use crate::math::vec::{Distance, Magnitude, V2f32};
use crate::math::vec::{Normalize, Vector2};

pub enum BorderBehaviourE {
    GoThrough,
    Reflect,
}

pub trait BorderBehaviour {
    fn border(&mut self, e: &BorderBehaviourE);
}
pub trait Behaviour: Send + Sync {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32;
}

pub struct AlignBehaviour;
impl Behaviour for AlignBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::ALLIGN) } {
            return V2f32::zero();
        }
        log::info!("Other boids : {:?}", other_boids);
        let mut avarage_velocity: V2f32 = other_boids
            .iter()
            .filter_map(|boid| match boid.id == self_boid.id {
                true => None,
                false => Some(boid.velocity),
            })
            .sum::<V2f32>();
        log::info!("avarage_velocity in Align{ } ", avarage_velocity);

        if avarage_velocity != Vector2::zero() {
            avarage_velocity /= (other_boids.len() - 1) as f32;
            avarage_velocity.set_magnitude(MAX_BOID_SPEED);
            avarage_velocity -= self_boid.velocity;
            avarage_velocity *= BehaviourConsts::ALLIGN_FACTOR;
        }
        avarage_velocity
    }
}
pub struct CohesionBehaviour;
impl Behaviour for CohesionBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::COHESION) } {
            return V2f32::zero();
        }
        let mut avarage_position: V2f32 = other_boids
            .iter()
            .filter_map(|boid| match boid.id == self_boid.id {
                true => None,
                false => Some(boid.position),
            })
            .sum::<V2f32>();

        if avarage_position != V2f32::zero() {
            avarage_position /= (other_boids.len() - 1) as f32;
            avarage_position -= self_boid.position;
            avarage_position.set_magnitude(MAX_BOID_SPEED);
            avarage_position -= self_boid.velocity;
            avarage_position *= BehaviourConsts::COHESION_FACTOR;
        }
        avarage_position
    }
}
pub struct SeperateBehaviour;
impl Behaviour for SeperateBehaviour {
    fn calculate(&self, self_boid: &Boid, other_boids: &[Boid]) -> V2f32 {
        if !unsafe { BEHAVIOUR_ENABLED.contains(BehaviourEnabled::SEPERATE) } {
            return V2f32::zero();
        }
        let mut avarage_position = V2f32::zero();
        let mut other = 0;
        for b in other_boids {
            if b.id != self_boid.id {
                let distance = V2f32::distance(self_boid.position, b.position);
                let mut diff = self_boid.position - b.position;
                diff /= (distance * distance);
                avarage_position += diff;
                other += 1;
            }
        }
        if other > 0 {
            avarage_position.set_magnitude(MAX_BOID_SPEED);
            avarage_position -= self_boid.velocity;
            avarage_position *= BehaviourConsts::SEPERATE_FACTOR;
        }
        avarage_position
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
            Vector2::new(
                (VIEW_PORT_SIZE.x - 100.0) as f32,
                (VIEW_PORT_SIZE.y - 100.0) as f32,
            ),
        );
        let x = if self_boid.position.x < r.left_up.x {
            BehaviourConsts::BOUND_FACTOR
        } else if self_boid.position.x > r.right_down.x {
            -BehaviourConsts::BOUND_FACTOR
        } else {
            0.0
        };

        let y = if self_boid.position.y < r.left_up.y {
            BehaviourConsts::BOUND_FACTOR
        } else if self_boid.position.y > r.right_down.y {
            -BehaviourConsts::BOUND_FACTOR
        } else {
            0.0
        };
        Vector2::new(x, y)
    }
}
