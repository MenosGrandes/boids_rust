use sdl2::{gfx::primitives::DrawRenderer, rect::Rect, render::WindowCanvas};

use super::traits::*;
use crate::{
    constants::{
        types::BoidId, BOID_COLOR, BOID_ID_ITERATOR, BOID_SIZE, BORDER_BEHAVIOUR, DRAW_VIEW,
        MAX_BOID_FORCE, VIEW_COLOR,
    },
    graphics::renderer::Renderable,
    logic::behaviour::traits::BorderBehaviour,
    math::{quadtree::region::Region, vec::V2f32},
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Boid {
    pub position: V2f32,
    pub velocity: V2f32,
    pub acceleration: V2f32,
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
    pub fn new(position: V2f32, velocity: V2f32, acceleration: V2f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            id: BOID_ID_ITERATOR.with(|id| id.borrow_mut().get_next()),
        }
    }
}

impl Renderable for Boid {
    fn render(&mut self, canvas: &mut WindowCanvas){
        canvas.set_draw_color(BOID_COLOR);
        DRAW_VIEW.with(|value: &std::cell::RefCell<bool>| {
            if *value.borrow() {
                canvas.set_draw_color(VIEW_COLOR);
                let r = Region::rect_from_center(self.position);
                let _ = canvas.draw_rect(Rect::from(r));
            }
        });
        canvas.set_draw_color(BOID_COLOR);
        let boid_rect =
            Region::rect_from_center_with_distance(self.position, (BOID_SIZE * 2) as f32);
        let _ = canvas.draw_rect(Rect::from(boid_rect));
    }
}
impl Updatable for Boid {
    fn update(&mut self) {
        self.velocity += self.acceleration * MAX_BOID_FORCE;
        self.position += self.velocity;
        self.acceleration = V2f32::zero();
        BORDER_BEHAVIOUR.with(|beh| self.border(&beh.borrow()));
        if self.position.x < 0.0 || self.position.y < 0.0
        {
            log::error!(" 2 position cannot be {:?} velocity {:?}", self.position, self.velocity);
            panic!("2 position cannot be {:?}, velocity {:?}", self.position, self.velocity);
        }
    }
}
