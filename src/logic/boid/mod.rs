use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::Color,
    render::WindowCanvas,
};

use crate::{
    constants::{BOID_SIZE, SCREEN_SIZE},
    graphics::renderer::Renderable,
    math::vec::*,
};

trait AllignBehaviour {
    const DISTANCE: f32;
    fn align(&mut self);
}
#[derive(PartialEq)]
pub struct Boid {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    color: Color,
    size: i16,
}
impl Boid {
    pub fn new(
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        acceleration: Vector2<f32>,
        color: Color,
        size: i16,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            color,
            size,
        }
    }

    pub fn draw_boid(&self, canvas: &WindowCanvas) -> Result<(), String> {
        canvas.filled_circle(
            self.position.x as i16,
            self.position.y as i16,
            self.size,
            self.color.as_rgba(),
        )?;
        Ok(())
    }
    pub fn update(&mut self) {
        if self.position.x as u32 > (SCREEN_SIZE.x - BOID_SIZE as u32) {
            //self.position.x = 0.0//reflect(Vector2::new(0.0,-1.0));
            self.velocity = self.velocity.reflect(Vector2::new(-1.0, 0.0));
        } else if self.position.x <= BOID_SIZE as f32 {
            self.velocity = self.velocity.reflect(Vector2::new(1.0, 0.0));
        } else if self.position.y >= (SCREEN_SIZE.y - BOID_SIZE as u32) as f32 {
            self.velocity = self.velocity.reflect(Vector2::new(0.0, 1.0));
        } else if self.position.y <= BOID_SIZE as f32 {
            self.velocity = self.velocity.reflect(Vector2::new(0.0, -1.0));
        }
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + self.acceleration;
    }
}

pub struct BoidManager {
    pub boids: Vec<Boid>,
}
impl BoidManager {
    pub fn new() -> Self {
        Self { boids: Vec::new() }
    }

    pub fn spawn_boid(&mut self, amount: i16) {
        /*
        let mut rng = rand::thread_rng();
        let x = Uniform::from(BOID_SIZE as u32..BOARD_SIZE.x);
        let y = Uniform::from(BOID_SIZE as u32..BOARD_SIZE.y);
        */
        for _i in 0..amount {
            /*
            let pos: Vector2<f32> = Vector2 {
                x: x.sample(&mut rng) as f32,
                y: y.sample(&mut rng) as f32,
            };*/
            self.boids.push(Boid::new(
                Vector2::new((SCREEN_SIZE.x / 2) as f32, (SCREEN_SIZE.y / 2) as f32),
                random(-10.0, 10.0),
                Vector2::zero(),
                random_color(),
                BOID_SIZE,
            ));
        }
    }
    pub fn remove_all_boids(&mut self) {
        self.boids = Vec::new();
    }
    pub fn update_all(&mut self) {
        for b in self.boids.iter_mut() {
            b.update();
        }
        self.align();
    }
}

impl Default for BoidManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Boid {
    fn render(&mut self, canvas: &WindowCanvas) -> Result<(), String> {
        self.draw_boid(&canvas)?;
        Ok(())
    }
}
impl AllignBehaviour for BoidManager {
    fn align(&mut self) {
        let mut avg: Vector2<f32> = Vector2::zero();
        let mut amount = 0;

        /*
        let mut b = self.boids.iter_mut(); //first boid
        let avg = 'align: loop {
            match b.next() {
                Some(current_boid) => {
                    for other_boid in self.boids.iter() {
                        let c = Vector2::distance(current_boid.position, other_boid.position);
                        if current_boid.position == other_boid.position {
                            break;
                        }
                        println!("{} distance = ",c);
                        if c.x.abs() < Self::DISTANCE && c.y.abs() < Self::DISTANCE {
                            avg = avg + other_boid.velocity;
                            amount += 1;
                        }
                    }
                    let outcome = current_boid.velocity + avg;
                    let ret =  outcome / amount as f32;
                }
                None => {
                    println!("{} and amount {}", avg, amount);
                    break 'align;
                }
            };
        };*/

    }

    const DISTANCE: f32 = 50.0;
}
