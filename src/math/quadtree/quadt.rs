use std::mem;

use crate::constants::types::{AreaId};
use crate::constants::{AREA_ID_ITERATOR};
use crate::logic::boid::boid_impl::Boid;
use crate::{graphics::renderer::Renderable, math::vec::random_color};

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
        id: AreaId,
        boundary: Region,
        boids: Vec<Boid>,
    },
    Root {
        neighbours: [Box<QuadTree>; 4],
    },
}
impl Renderable for QuadTree {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(random_color());

        match self {
            QuadTree::Leaf {
                boundary,
                boids: _,
                id: _,
            } => {
                let _ = canvas.draw_rect(rect!(
                    boundary.left_up.x,
                    boundary.left_up.y,
                    boundary.width_height.x,
                    boundary.width_height.y
                ));
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    n.render(canvas)?;
                }
            }
        }

        Ok(())
    }
}
impl QuadTree {
    const MAX_CAPACITY: usize = 20;

    pub fn new(boundary: Region) -> Self {
        QuadTree::Leaf {
            boundary,
            boids: Vec::new(),
            id: AREA_ID_ITERATOR.with(|n| n.borrow_mut().get_next()),
        }
    }

    pub fn count(&self) -> usize {
        match self {
            QuadTree::Leaf {
                boundary: _,
                boids: points,
                id: _,
            } => return points.len(),
            QuadTree::Root { neighbours } => neighbours.into_iter().map(|n| n.count()).sum(),
        }
    }

    pub fn insert(&mut self, boid: Boid) -> Result<AreaId, &str> {
        match self {
            QuadTree::Leaf {
                boundary,
                boids,
                id,
            } => {
                if !boundary.contains(&boid) {
                    return Err("Boundary doesn't contain boid");
                } else if boids.len() == QuadTree::MAX_CAPACITY {
                    self.subdivide();
                    return self.insert(boid);
                } else {
                    boids.push(boid);
                    return Ok(*id);
                }
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    let ok = n.insert(boid);
                    if ok.is_ok() {
                        return ok;
                    }
                }
                return Err("Boid couldn't be inserted in any sub-tree");
            }
        }
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf {
                boundary,
                boids: points,
                id: _,
            } => {
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
                let _ = mem::replace(self, new);
            }
            _ => {}
        }
    }
}
