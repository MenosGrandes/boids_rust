use crate::{
    graphics::renderer::Renderable,
    logic::boid::Boid,
    math::vec::{random_color, Distance, V2usize},
};
use sdl2::rect::Rect;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
pub trait SubInto<T> {
    fn sub_into(data: &T) -> [T; 4];
}
#[derive(Clone, Debug)]
pub struct Region {
    pub left_up: V2usize,
    pub right_down: V2usize,
    pub width_height: V2usize,
}

impl Region {
    pub fn new(left_up: V2usize, right_down: V2usize) -> Self {
        let height = right_down.y - left_up.y;
        let width = right_down.x - left_up.x;
        Self {
            left_up,
            right_down,
            width_height: V2usize::new(width, height),
        }
    }

    pub fn contains(&self, boid: &Boid) -> bool {
        boid.position.x > self.left_up.x as f32
            && boid.position.x < self.right_down.x as f32
            && boid.position.y > self.left_up.y as f32
            && boid.position.y < self.right_down.y as f32
    }
}
impl Renderable for Region {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(random_color());
        let _ = canvas.draw_rect(rect!(
            self.left_up.x,
            self.left_up.y,
            self.width_height.x,
            self.width_height.y
        ));
        Ok(())
    }
}
impl SubInto<Region> for Region {
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
        let height = data.width_height.y / 2;
        let width = data.width_height.x / 2;
        let center = V2usize::new(data.left_up.x + width, data.left_up.y + height);
        [
            Region::new(data.left_up, center),
            Region::new(
                V2usize::new(center.x, data.left_up.y),
                V2usize::new(data.right_down.x, center.y),
            ),
            Region::new(
                V2usize::new(data.left_up.x, center.y),
                V2usize::new(center.x, data.right_down.y),
            ),
            Region::new(center, data.right_down),
        ]
    }
}
