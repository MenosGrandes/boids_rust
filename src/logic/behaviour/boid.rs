use crate::logic::boid::boid_impl::Boid;
use crate::{constants::*, math::vec::*};

use super::traits::{BorderBehaviour, BorderBehaviourE};

impl BorderBehaviour for Boid {
    fn border(&mut self, e: &BorderBehaviourE) {
        match e {
            BorderBehaviourE::Reflect => {
                if self.position.x > VIEW_PORT_SIZE.x as f32 - (BOID_SIZE as f32) * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(-1.0, 0.0));
                } else if self.position.x < BOID_SIZE as f32 * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(1.0, 0.0));
                }
                if self.position.y > VIEW_PORT_SIZE.y as f32 - (BOID_SIZE as f32) * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, 1.0));
                } else if self.position.y < BOID_SIZE as f32 * 3.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, -1.0));
                }
            }
            BorderBehaviourE::GoThrough => {
                if self.position.x > VIEW_PORT_SIZE.x as f32 - (BOID_SIZE as f32) * 2.0 {
                    self.position.x = BOID_SIZE as f32 * 2.0;
                } else if self.position.x < BOID_SIZE as f32 * 2.0 {
                    self.position.x = VIEW_PORT_SIZE.x as f32 - (BOID_SIZE as f32) * 2.0;
                }
                if self.position.y > VIEW_PORT_SIZE.y as f32 - (BOID_SIZE as f32) * 2.0 {
                    self.position.y = BOID_SIZE as f32 * 2.0;
                } else if self.position.y < BOID_SIZE as f32 * 2.0 {
                    self.position.y = VIEW_PORT_SIZE.y as f32 - (BOID_SIZE as f32) * 2.0;
                }
            }
        }
    }
}
