use std::mem;

use crate::constants::types::AreaId;
use crate::constants::{AREA_ID_ITERATOR, MAX_BOID_IN_AREA};
use crate::logic::boid::boid_impl::Boid;
use crate::{graphics::renderer::Renderable, math::vec::random_color};

use super::region::Region;
use super::traits::{Intersect, SubInto};

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

    pub fn insert(&mut self, boid: Boid, len: &mut usize) -> Result<AreaId, String> {
        match self {
            QuadTree::Leaf {
                boundary,
                boids,
                id,
            } => {
                if !boundary.contains_boid(&boid) {
                    return Err("Boundary doesn't contain boid".to_string());
                } else if boids.len() == MAX_BOID_IN_AREA {
                    *len = boids.len() + *len;
                    self.subdivide();
                    return self.insert(boid, len);
                } else {
                    boids.push(boid);
                    return Ok(*id);
                }
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    let ok = n.insert(boid, len);
                    if ok.is_ok() {
                        return ok;
                    }
                }
                return Err(format!("Boid couldn't be inserted in any sub-tree {}", len));
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
                let mut len = 0;
                for p in points {
                    new.insert(*p, &mut len).unwrap();
                }
                let _ = mem::replace(self, new);
            }
            _ => {}
        }
    }
    pub fn get_all_boids_in_boundry(&self, query_boundry: &Region, found_boids: &mut Vec<Boid>) {
        match self {
            QuadTree::Leaf {
                id: _,
                boundary,
                boids,
            } => {
                if !query_boundry.intersect_with(boundary) {
                    return;
                }
                for b in boids {
                    if boundary.contains_boid(b) {
                        found_boids.push(*b);
                    }
                }
            }

            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    n.get_all_boids_in_boundry(query_boundry, found_boids);
                }
            }
        }
    }
}
