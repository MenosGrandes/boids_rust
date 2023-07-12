use crate::{
    constants::*,
    logic::{behaviour::traits::SeeBehaviour, boid::Boid},
    math::vec::*,
};

use super::traits::{BorderBehaviour, BorderBehaviourE, FlockBehaviour};

impl SeeBehaviour for Boid {
    fn get_other_visible(self, other: &[Boid]) -> Vec<Boid> {
        let mut other_visible_boids: Vec<Boid> = Vec::new();
        for other_boid in other {
            if self.id == other_boid.id {
                break;
            }
            let c = Vector2::distance(self.position, other_boid.position);

            if c.x.abs() < VIEW_DISTANCE && c.y.abs() < VIEW_DISTANCE {
                other_visible_boids.push(*other_boid);
            }
        }
        other_visible_boids
    }
}
impl FlockBehaviour for Boid {
    fn align(&self, other_boids: &[Boid]) -> V2f32 {
        if other_boids.len() > 0 {
            let mut avarage_velocity: V2f32 =
                other_boids.iter().map(|boid| boid.velocity).sum::<V2f32>()
                    / (other_boids.len() as f32);

            avarage_velocity.set_magnitude(MAX_BOID_SPEED);
            avarage_velocity -= self.velocity;
            avarage_velocity *= ALLIGN_FACTOR;
            return avarage_velocity;
        }
        V2f32::zero()
    }

    //MenosGrandes to lookup
    fn cohesion(&self, other_boids: &[Boid]) -> V2f32 {
        if other_boids.len() > 0 {
            let mut avarage_position: V2f32 =
                other_boids.iter().map(|boid| boid.position).sum::<V2f32>()
                    / (other_boids.len() as f32);

            avarage_position -= self.position;
            avarage_position.set_magnitude(MAX_BOID_SPEED);
            avarage_position -= self.velocity;
            avarage_position *= COHESION_FACTOR;

            return avarage_position;
        }
        V2f32::zero()
    }

    fn seperate(&self, other_boids: &[Boid]) -> V2f32 {
        if other_boids.len() > 0 {
            let mut avarage_position: V2f32 = other_boids
                .iter()
                .map(|boid| {
                    (self.position - boid.position)
                        / (Vector2::distance(self.position, boid.position)
                            * Vector2::distance(self.position, boid.position))
                })
                .sum::<V2f32>()
                / (other_boids.len() as f32);

            avarage_position.set_magnitude(MAX_BOID_SPEED);
            avarage_position -= self.velocity;
            avarage_position *= SEPERATE_FACTOR;

            return avarage_position;
        }
        V2f32::zero()
    }
}

impl BorderBehaviour for Boid {
    fn border(&mut self, e: &BorderBehaviourE) {
        match e {
            BorderBehaviourE::Reflect => {
                if self.position.x > SCREEN_SIZE.x as f32 {
                    self.velocity = self.velocity.reflect(Vector2::new(-1.0, 0.0));
                } else if self.position.x < 0.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(1.0, 0.0));
                }
                if self.position.y > SCREEN_SIZE.y as f32 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, 1.0));
                } else if self.position.y < 0.0 {
                    self.velocity = self.velocity.reflect(Vector2::new(0.0, -1.0));
                }
            }
            BorderBehaviourE::GoThrough => {
                if self.position.x > SCREEN_SIZE.x as f32 {
                    self.position.x = 0.0;
                } else if self.position.x < 0.0 {
                    self.position.x = SCREEN_SIZE.x as f32;
                }
                if self.position.y > SCREEN_SIZE.y as f32 {
                    self.position.y = 0.0;
                } else if self.position.y < 0.0 {
                    self.position.y = SCREEN_SIZE.y as f32;
                }
            }
        }
    }
}
