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
    Leaf { boundary: Region, boids: Vec<Boid> },
    Root { neighbours: [Box<QuadTree>; 4] },
}
impl Renderable for QuadTree {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(random_color());

        match self {
            QuadTree::Leaf {
                boundary,
                boids,
            } => {
                let _ = canvas.draw_rect(rect!(
                    boundary.left_up.x,
                    boundary.left_up.y,
                    boundary.width_height.x,
                    boundary.width_height.y
                ));
                for b in boids
                {
                    b.render(canvas);
                }
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    n.render(canvas);
                }
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
            boids: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        match self {
            QuadTree::Leaf {
                boundary: _,
                boids: points,
            } => return points.len(),
            QuadTree::Root { neighbours } => neighbours.into_iter().map(|n| n.count()).sum(),
        }
    }

    pub fn insert(&mut self, point: Boid) -> Result<(), &str> {
        match self {
            QuadTree::Leaf { boundary, boids: points } => {
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
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    if n.insert(point).is_ok() {
                        return Ok(());
                    }
                }
                /*
                if ne.insert(point).is_ok() {
                    return Ok(());
                } else if se.insert(point).is_ok() {
                    return Ok(());
                } else if sw.insert(point).is_ok() {
                    return Ok(());
                } else if nw.insert(point).is_ok() {
                    return Ok(());
                }*/
                return Err("Point couldn't be inserted in any sub-tree");
            }
        }
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf { boundary, boids: points } => {
                let b = Region::sub_into(&boundary);

                let nei: [Box<QuadTree>; 4] = b
                    .into_iter()
                    .map(|r| Box::new(QuadTree::new(r.clone())))
                    .collect::<Vec<Box<QuadTree>>>()
                    .try_into()
                    .unwrap();

                let mut new = QuadTree::Root { neighbours: nei };
                for p in points {
                    new.insert(*p).unwrap();
                }
                mem::replace(self, new);
            }
            _ => {}
        }
    }
}
