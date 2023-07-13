use crate::{
    graphics::renderer::Renderable,
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
pub struct Region {
    pub left_up: V2usize,
    pub right_down: V2usize,
}

impl Region {
    pub fn new(left_up: V2usize, right_down: V2usize) -> Self {
        Self {
            left_up,
            right_down,
        }
    }
}
impl Renderable for Region {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(random_color());
        let height = self.right_down.y - self.left_up.y;
        let width = self.right_down.x - self.left_up.x;
        let _ = canvas.draw_rect(rect!(self.left_up.x, self.left_up.y, width, height));
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
        println!(
            "sub_into rd = {:?}, lu = {:?}",
            data.right_down, data.left_up
        );
        let height = (data.right_down.y - data.left_up.y) / 2;
        println!("hei {:?}", height);
        let width = (data.right_down.x - data.left_up.x) / 2;
        println!("wi {:?}", width);
        let center = V2usize::new(data.left_up.x + width, data.left_up.y + height);
        println!("center {:?}", center);
        [
            Region::new(data.left_up, center),
            Region::new(V2usize::new(center.x, data.left_up.y), V2usize::new(data.right_down.x, center.y)),
            Region::new(V2usize::new(data.left_up.x, center.y), V2usize::new(center.x, data.right_down.y)),
            Region::new(center, data.right_down),
        ]
    }
}
