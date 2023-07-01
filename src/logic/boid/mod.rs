use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::Color,
    render::WindowCanvas,
};

use crate::{
    constants::{BOID_SIZE, SCREEN_SIZE},
    graphics::renderer::Renderable,
    math::vec::{random, random_color, Vector2},
};

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
        if self.position.x as u32 > SCREEN_SIZE.x {
            self.velocity = self.velocity * -1.0;
            self.acceleration = self.acceleration * -1.0;
        }
        if self.position.x <= 0.0 {
            self.velocity = self.velocity * -1.0;
            self.acceleration = self.acceleration * -1.0;
        }
        self.position = self.position + self.velocity;
        //self.velocity = self.velocity + self.acceleration;
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
                random(),
                random(),
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
