use crate::{
    camera::Camera,
    constants::{REGION_COLOR, VIEW_DISTANCE},
    graphics::renderer::Renderable,
    logic::boid::boid_impl::Boid,
    math::quadtree::traits::SubInto,
    math::vec::{V2f32, Vector2},
};
use sdl2::rect::Rect;

use super::traits::Intersect;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
#[derive(Clone, Debug)]
pub struct Region {
    pub left_up: V2f32,
    pub right_down: V2f32,
    pub width_height: V2f32,
}
impl Default for Region {
    fn default() -> Self {
        Self::new(Vector2::zero(), Vector2::zero())
    }
}

impl Region {
    pub fn rect_from_center_with_distance(center: V2f32, view_distance: f32) -> Self {
        let diagonal = (view_distance * f32::sqrt(2.0)) / 2.0;
        Self {
            left_up: center - diagonal / 2.0,
            right_down: center + diagonal / 2.0,
            width_height: V2f32::new(diagonal, diagonal),
        }
    }
    pub fn rect_from_center(center: V2f32) -> Self {
        Region::rect_from_center_with_distance(center, VIEW_DISTANCE)
    }
    pub fn new(left_up: V2f32, right_down: V2f32) -> Self {
        let height = right_down.y - left_up.y;
        let width = right_down.x - left_up.x;
        Self {
            left_up,
            right_down,
            width_height: V2f32::new(width, height),
        }
    }
    pub fn get_center_point(&self) -> V2f32 {
        V2f32::new(
            self.left_up.x + self.width_height.x / 2.0,
            self.left_up.y + self.width_height.y / 2.0,
        )
    }
    pub fn is_empty(&self) -> bool {
        return self.width_height.x == 0.0 || self.width_height.y == 0.0;
    }

    pub fn contains_boid(&self, boid: &Boid) -> bool {
        boid.position.x > self.left_up.x
            && boid.position.x < self.right_down.x
            && boid.position.y > self.left_up.y
            && boid.position.y < self.right_down.y
    }
}
impl Renderable for Region {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas, camera: &Camera) {
        canvas.set_draw_color(REGION_COLOR);
        let _ = canvas.draw_rect(rect!(
            self.left_up.x,
            self.left_up.y,
            self.width_height.x,
            self.width_height.y
        ));
    }
}
impl SubInto for Region {
    /*
    *|=>left_up
    *|-------|
     |   |   |
     | 1 | 2 |
     |   |   |
     |___|___|
     |   |   |
     | 3 | 4 |
     |   |   |
     |___|___|
             | => right_down
    *
    *
    *
    * */
    fn sub_into(data: &Region) -> [Region; 4] {
        let height = data.width_height.y / 2.0;
        let width = data.width_height.x / 2.0;
        let center = Vector2::new(data.left_up.x + width, data.left_up.y + height);
        [
            Region::new(data.left_up, center),
            Region::new(
                Vector2::new(center.x, data.left_up.y),
                Vector2::new(data.right_down.x, center.y),
            ),
            Region::new(
                Vector2::new(data.left_up.x, center.y),
                Vector2::new(center.x, data.right_down.y),
            ),
            Region::new(center, data.right_down),
        ]
    }
}
impl Intersect for Region {
    fn intersect_with(&self, other: &Self) -> bool {
        #[cfg(debug_assertions)]
        if self.is_empty() || other.is_empty() {
            return false;
        }

        return self.left_up.x <= (other.right_down.x)
            && (self.right_down.x) >= other.left_up.x
            && self.left_up.y <= (other.right_down.y)
            && (self.right_down.y) >= other.left_up.y;
    }
}
/*
impl From<Rect> for Region
{
    fn from(value: Rect) -> Self {
        todo!()
        let przekatna = (view_distance * f32::sqrt(2.0)) / 2.0;
        Region{
            left_up: center - przekatna / 2.0,
            right_down: center + przekatna / 2.0,
            width_height: V2f32::new(przekatna, przekatna),
        }
    }
}*/
impl From<Region> for Rect {
    fn from(value: Region) -> Self {
        Rect::new(
            value.left_up.x as i32,
            value.left_up.y as i32,
            value.width_height.x as u32,
            value.width_height.y as u32,
        )
    }
}
#[test]
fn empty_region() {
    let _r = Region::new(V2f32::new(0.0, 0.0), V2f32::new(0.0, 0.0));
    assert_eq!(_r.is_empty(), true);
    let _r2 = Region::new(V2f32::new(100.0, 0.0), V2f32::new(0.0, 0.0));
    assert_eq!(_r2.is_empty(), true);
}

#[test]
fn region_intersects() {
    let r_1 = Region::new(V2f32::new(0.0, 0.0), V2f32::new(200.0, 200.0));
    let r_2 = Region::new(V2f32::new(199.0, 100.0), V2f32::new(340.0, 600.0));
    assert_eq!(r_1.intersect_with(&r_2), true);
    assert_eq!(r_2.intersect_with(&r_1), true);
}

#[test]
fn region_intersects_1() {
    let r_1 = Region::new(V2f32::new(0.0, 0.0), V2f32::new(200.0, 200.0));
    let r_2 = Region::new(V2f32::new(100.0, 100.0), V2f32::new(340.0, 600.0));
    assert_eq!(r_1.intersect_with(&r_2), true);
    assert_eq!(r_2.intersect_with(&r_1), true);
}

#[test]
fn region_intersects_2() {
    let r_1 = Region::new(V2f32::new(0.0, 0.0), V2f32::new(200.0, 200.0));
    let r_2 = Region::new(V2f32::new(300.0, 100.0), V2f32::new(340.0, 600.0));
    assert_eq!(r_1.intersect_with(&r_2), false);
    assert_eq!(r_2.intersect_with(&r_1), false);
}
#[test]
fn region_intersects_3() {
    let r_1 = Region::new(V2f32::new(0.0, 0.0), V2f32::new(200.0, 200.0));
    let r_2 = Region::new(V2f32::new(0.0, 201.0), V2f32::new(340.0, 600.0));
    assert_eq!(r_1.intersect_with(&r_2), false);
    assert_eq!(r_2.intersect_with(&r_1), false);
}
