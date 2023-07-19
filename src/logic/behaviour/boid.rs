use crate::logic::boid::boid_impl::Boid;
use crate::{constants::*, math::vec::*};

use super::traits::{BorderBehaviour, BorderBehaviourE, SeeBehaviour};

impl SeeBehaviour for Boid {
    fn get_other_visible(self, other: &[Boid]) -> Vec<Boid> {
        let mut other_visible_boids: Vec<Boid> = Vec::new();
        for other_boid in other {
            if self.id == other_boid.id {
                break;
            }
            let distance = Vector2::distance(self.position, other_boid.position);
            if Vector2::in_between(distance, VIEW_DISTANCE) {
                other_visible_boids.push(*other_boid);
            }
        }
        other_visible_boids
    }
}

impl BorderBehaviour for Boid {
    fn border(&mut self, e: &BorderBehaviourE) {
        match e {
            BorderBehaviourE::Reflect => {
                if self.position.x > SCREEN_SIZE.x as f32 - (BOID_SIZE as f32) * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(-1.0, 0.0));
                } else if self.position.x < BOID_SIZE as f32 * 3.0{
                    self.velocity = self.velocity.reflect(Vector2::new(1.0, 0.0));
                }
                if self.position.y > SCREEN_SIZE.y as f32 - (BOID_SIZE as f32) * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, 1.0));
                } else if self.position.y < BOID_SIZE as f32 * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, -1.0));
                }
            }
            BorderBehaviourE::GoThrough => {
                if self.position.x > SCREEN_SIZE.x as f32 - (BOID_SIZE as f32) * 2.0 {
                    self.position.x = BOID_SIZE as f32 * 2.0;
                } else if self.position.x < BOID_SIZE as f32 * 2.0 {
                    self.position.x = SCREEN_SIZE.x as f32 - (BOID_SIZE as f32) * 2.0;
                }
                if self.position.y > SCREEN_SIZE.y as f32 - (BOID_SIZE as f32) * 2.0 {
                    self.position.y = BOID_SIZE as f32 * 2.0;
                } else if self.position.y < BOID_SIZE as f32 * 2.0 {
                    self.position.y = SCREEN_SIZE.y as f32 - (BOID_SIZE as f32) * 2.0;
                }
            }
        }
    }
}
