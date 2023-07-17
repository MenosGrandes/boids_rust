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
        BOID_ID_ITERATOR, BOID_SIZE, BORDER_BEHAVIOUR, DRAW_VIEW, MAX_BOID_FORCE, VIEW_DISTANCE,
    },
    graphics::renderer::Renderable,
    logic::behaviour::traits::BorderBehaviour,
    math::{vec::V2f32, quadtree::region::Region},
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Boid {
    pub position: V2f32,
    pub velocity: V2f32,
    pub acceleration: V2f32,
    color: Color,
    pub id: BoidId,
    pub area_id: AreaId,
}
/*
impl std::fmt::Debug for Boid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Boid")
            .field("id", &self.id)
            .field("pos", &self.position)
            .finish()
    }
}*/
impl Boid {
    pub fn new(position: V2f32, velocity: V2f32, acceleration: V2f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            color,
            id: BOID_ID_ITERATOR.with(|id| id.borrow_mut().get_next()),
            area_id: 0,
        }
    }

    pub fn draw_boid(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(self.color.as_rgba());
        DRAW_VIEW.with(|value: &std::cell::RefCell<bool>| {
            if *value.borrow() {
                let r = Region::rect_from_center(self.position, VIEW_DISTANCE);
                let rect = Rect::new(
                    r.left_up.x as i32, r.left_up.y as i32,
                    r.width_height.x as u32,
                    r.width_height.y as u32,
                );
                let _ = canvas.draw_rect(rect);
            }
        });
        canvas.filled_circle(
            self.position.x as i16,
            self.position.y as i16,
            BOID_SIZE,
            self.color.as_rgba(),
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
