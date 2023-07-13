use std::mem;

use crate::{graphics::renderer::Renderable, logic::boid::Boid, math::vec::random_color};

use super::region::{Region, SubInto};

use sdl2::rect::Rect;
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
#[derive(Debug)]
pub enum QuadTree {
    Leaf {
        boundary: Region,
        points: Vec<Boid>,
    },
    Root {
        ne: Box<QuadTree>,
        se: Box<QuadTree>,
        sw: Box<QuadTree>,
        nw: Box<QuadTree>,
    },
}
impl Renderable for QuadTree {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(random_color());

        match self {
            QuadTree::Leaf { boundary, points : _ } => {
                let _ = canvas.draw_rect(rect!(
                    boundary.left_up.x,
                    boundary.left_up.y,
                    boundary.width_height.x,
                    boundary.width_height.y
                ));
            }
            QuadTree::Root { ne, se, sw, nw } => {
                let _ = ne.render(canvas);
                let _ = se.render(canvas);
                let _ = sw.render(canvas);
                let _ = nw.render(canvas);
            }
        }

        Ok(())
    }
}
impl QuadTree {
    const MAX_CAPACITY: usize = 5;

    pub fn new(boundary: Region) -> Self {
        QuadTree::Leaf {
            boundary,
            points: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        match self {
            QuadTree::Leaf {
                boundary: _,
                points,
            } => return points.len(),
            QuadTree::Root { ne, se, sw, nw } => {
                return ne.count() + se.count() + sw.count() + nw.count()
            }
        }
    }

    pub fn insert(&mut self, point: Boid) -> Result<(), &str> {
        match self {
            QuadTree::Leaf { boundary, points } => {
                if !boundary.contains(&point) {
                    return Err("Boundary doesn't contain point");
                } else if points.len() == QuadTree::MAX_CAPACITY {
                    self.subdivide();
                    return self.insert(point);
                } else {
                    points.push(point);
                    return Ok(());
                }
            }
            QuadTree::Root { ne, se, sw, nw } => {
                if ne.insert(point).is_ok() {
                    return Ok(());
                } else if se.insert(point).is_ok() {
                    return Ok(());
                } else if sw.insert(point).is_ok() {
                    return Ok(());
                } else if nw.insert(point).is_ok() {
                    return Ok(());
                }
                return Err("Point couldn't be inserted in any sub-tree");
            }
        }
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf { boundary, points } => {
                let b = Region::sub_into(&boundary);
                let mut new = QuadTree::Root {
                    ne: Box::new(QuadTree::new(b[0].clone())),
                    se: Box::new(QuadTree::new(b[1].clone())),
                    sw: Box::new(QuadTree::new(b[2].clone())),
                    nw: Box::new(QuadTree::new(b[3].clone())),
                };
                for p in points {
                    new.insert(*p).unwrap();
                }
                mem::replace(self, new);
            }
            _ => {}
        }
    }
}
