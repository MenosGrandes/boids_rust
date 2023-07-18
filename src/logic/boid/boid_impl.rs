use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
};

use super::traits::*;
use crate::{
    constants::{
        types::{AreaId, BoidId},
        BOID_ID_ITERATOR, BOID_SIZE, BORDER_BEHAVIOUR, DRAW_VIEW, MAX_BOID_FORCE, VIEW_DISTANCE, BOID_COLOR,
    },
    graphics::renderer::Renderable,
    logic::behaviour::traits::BorderBehaviour,
    math::{quadtree::region::Region, vec::{V2f32, random_color}},
};

#[derive(PartialEq, Copy, Clone)]
pub struct Boid {
    pub position: V2f32,
    pub velocity: V2f32,
    pub acceleration: V2f32,
    pub id: BoidId,
}

impl std::fmt::Debug for Boid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Boid")
            .field("id", &self.id)
            .field("pos", &self.position)
            .finish()
    }
}
impl Boid {
    pub fn new(position: V2f32, velocity: V2f32, acceleration: V2f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            id: BOID_ID_ITERATOR.with(|id| id.borrow_mut().get_next()),
        }
    }

    pub fn draw_boid(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(BOID_COLOR);
        DRAW_VIEW.with(|value: &std::cell::RefCell<bool>| {
            if *value.borrow() {
                let r = Region::rect_from_center(self.position);
                let _ = canvas.draw_rect(Rect::from(r));
            }
        });
        canvas.filled_circle(
            self.position.x as i16,
            self.position.y as i16,
            BOID_SIZE,
            BOID_COLOR
        )?;
        Ok(())
    }
}

impl Renderable for Boid {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.draw_boid(canvas)?;
        Ok(())
    }
}
impl Updatable for Boid {
    fn update(&mut self) {
        self.velocity += self.acceleration * MAX_BOID_FORCE;
        self.position += self.velocity;
        self.acceleration = V2f32::zero();
        BORDER_BEHAVIOUR.with(|beh| self.border(&beh.borrow()));
    }
}
