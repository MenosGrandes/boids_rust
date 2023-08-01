use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use super::traits::*;
use crate::{
    camera::Camera,
    constants::{
        types::BoidId, DrawPrimitives, BOID_COLOR, BOID_ID_ITERATOR, BOID_SIZE, BORDER_BEHAVIOUR,
        DRAW_PRIMITIVES, MAX_BOID_FORCE, MAX_BOID_SPEED, VIEW_COLOR,
    },
    graphics::renderer::Renderable,
    logic::behaviour::traits::BorderBehaviour,
    math::{
        quadtree::region::Region,
        vec::{Magnitude, V2f32},
    },
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Boid {
    pub position: V2f32,
    pub velocity: V2f32,
    pub id: BoidId,
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
    pub fn new(position: V2f32, velocity: V2f32) -> Self {
        Self {
            position,
            velocity,
            id: BOID_ID_ITERATOR.with(|id| id.borrow_mut().get_next()),
        }
    }
}

impl Renderable for Boid {
    fn render(&mut self, canvas: &mut WindowCanvas, camera: &Camera) {
        canvas.set_draw_color(BOID_COLOR);
        DRAW_PRIMITIVES.with(|value| {
            if value.borrow().contains(DrawPrimitives::BOID_VIEW) {
                canvas.set_draw_color(VIEW_COLOR);
                let r = Region::rect_from_center(camera.calc_pos_v2f32(self.position));
                let _ = canvas.draw_rect(Rect::from(r));
            }
        });
        canvas.set_draw_color(BOID_COLOR);
        let r = Region::rect_from_center_with_distance(
            camera.calc_pos_v2f32(self.position),
            BOID_SIZE as f32,
        );
        let _ = canvas.rectangle(
            r.left_up.x as i16,
            r.left_up.y as i16,
            r.right_down.x as i16,
            r.right_down.y as i16,
            Color::BLUE,
        );
    }
}
impl UpdatableAcceleration for Boid {
    fn update(&mut self, acceleration: V2f32) {
        log::info!("UpdateAcceleration acceleration {:?}", acceleration);
        BORDER_BEHAVIOUR.with(|beh| self.border(&beh.borrow()));
        self.position += self.velocity;
        self.velocity += acceleration; // * MAX_BOID_FORCE;
        self.velocity.limit(MAX_BOID_SPEED);
        log::info!("update {:?}", self);
    }
}
