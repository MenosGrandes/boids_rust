use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
};

use crate::{
    constants::{
        BOID_SIZE, BORDER_BEHAVIOUR, DRAW_VIEW, MAX_BOID_FORCE, SCREEN_SIZE, VIEW_DISTANCE,
    },
    graphics::renderer::Renderable,
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{random_color, Magnitude, V2usize, Vector2},
    },
};

use super::behaviour::traits::{
    AlignBehaviour, Behaviour, BorderBehaviour, CohesionBehaviour, SeeBehaviour, SeperateBehaviour,
};
use crate::math::vec::V2f32;
pub trait Updatable {
    fn update(&mut self);
}

static mut BOID_ID: u32 = 0;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Boid {
    pub position: V2f32,
    pub velocity: V2f32,
    pub acceleration: V2f32,
    color: Color,
    size: i16,
    pub id: u32,
}
impl Boid {
    pub fn new(
        position: V2f32,
        velocity: V2f32,
        acceleration: V2f32,
        color: Color,
        size: i16,
    ) -> Self {
        unsafe {
            BOID_ID += 1;
        }
        Self {
            position,
            velocity,
            acceleration,
            color,
            size,
            id: unsafe { BOID_ID },
        }
    }

    pub fn draw_boid(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.color.as_rgba());
        DRAW_VIEW.with(|value: &std::cell::RefCell<bool>| {
            if *value.borrow() {
                let r = Rect::new(
                    (self.position.x as i32 - (VIEW_DISTANCE / 2.0) as i32) as i32,
                    (self.position.y as i32 - (VIEW_DISTANCE / 2.0) as i32) as i32,
                    (VIEW_DISTANCE) as u32,
                    (VIEW_DISTANCE) as u32,
                );
                let _ = canvas.draw_rect(r);
            }
        });
        canvas.filled_circle(
            self.position.x as i16,
            self.position.y as i16,
            self.size,
            self.color.as_rgba(),
        )?;
        Ok(())
    }
}

pub struct BoidManager {
    pub boids: Vec<Boid>,
    pub behaviours: Vec<Box<dyn Behaviour>>,
    pub quad_tree: QuadTree,
}
impl BoidManager {
    pub fn new(starting_region: Region) -> Self {
        Self {
            boids: Vec::new(),
            behaviours: vec![
                Box::new(AlignBehaviour {}),
                Box::new(SeperateBehaviour {}),
                Box::new(CohesionBehaviour {}),
            ],
            quad_tree: QuadTree::new(starting_region),
        }
    }

    pub fn add_boid(&mut self, amount: i16) {
        for _ in 0..amount {
            let mut c = Vector2::random(-1.0, 1.0); //
            c.set_magnitude(2.0);
            self.boids.push(Boid::new(
                Vector2::random_from_vec(
                    Vector2::new(0.0, SCREEN_SIZE.x as f32),
                    Vector2::new(0.0, SCREEN_SIZE.y as f32),
                ),
                c,
                Vector2::new(0.01, 0.01),
                random_color(),
                BOID_SIZE,
            ));
        }
    }
    pub fn spawn_boid(&mut self, amount: i16) {
        self.boids = Vec::with_capacity(amount as usize);
        self.add_boid(amount);
    }
    pub fn remove_all_boids(&mut self) {
        self.boids = Vec::new();
    }
}
impl Renderable for BoidManager {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for b in self.boids.iter_mut() {
            b.render(canvas)?;
        }
        self.quad_tree.render(canvas)?;
        Ok(())
    }
}

impl Default for BoidManager {
    fn default() -> Self {
        Self::new(Region::default())
    }
}

impl Renderable for Boid {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.draw_boid(canvas)?;
        Ok(())
    }
}
impl Updatable for BoidManager {
    fn update(&mut self) {
        let r: Region = Region::new(
            V2usize::new(0, 0),
            V2usize::new(SCREEN_SIZE.x as usize, SCREEN_SIZE.y as usize),
        );
        self.quad_tree = QuadTree::new(r.clone());
        for b in &self.boids {
            let _ = self.quad_tree.insert(b.clone());
        }
        for i in 0..(self.boids).len() {
            let mut b = self.boids[i];
            let other_visible_boids = b.get_other_visible(&self.boids);

            if other_visible_boids.len() > 0 {
                for behaviour in &self.behaviours {
                    b.acceleration += behaviour.calculate(&b, &other_visible_boids);
                }
            }
            b.update();
            b.acceleration = V2f32::zero();
            self.boids[i] = b;
        }
    }
}
impl Updatable for Boid {
    fn update(&mut self) {
        self.velocity += self.acceleration * MAX_BOID_FORCE;
        self.position += self.velocity;
        BORDER_BEHAVIOUR.with(|beh| self.border(&beh.borrow()));
    }
}
