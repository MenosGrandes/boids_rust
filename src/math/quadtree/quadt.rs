use std::mem;

use crate::constants::{MAX_BOID_IN_AREA, QUAD_TREE_COLOR, BOID_SIZE};
use crate::logic::boid::boid_impl::Boid;

use crate::graphics::renderer::Renderable;
use crate::math::vec::Vector2;

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
    Leaf { boundary: Region, boids: Vec<Boid> },
    Root { neighbours: [Box<QuadTree>; 4] },
}
impl Renderable for QuadTree {
    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas){
        canvas.set_draw_color(QUAD_TREE_COLOR);

        match self {
            QuadTree::Leaf { boundary, boids: _ } => {
                let _ = canvas.draw_rect(rect!(
                    boundary.left_up.x,
                    boundary.left_up.y,
                    boundary.width_height.x,
                    boundary.width_height.y
                ));
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    n.render(canvas);
                }
            }
        }
    }
}
impl QuadTree {
    pub fn new(boundary: Region) -> Self {
        QuadTree::Leaf {
            boundary,
            boids: Vec::with_capacity(MAX_BOID_IN_AREA),
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

    pub fn insert(&mut self, boid: Boid) -> Result<(), &'static str> {
        log::debug!("Insert {:?}", boid);
        match self {
            QuadTree::Leaf { boundary, boids } => {
                log::info!("its leaf");
                if !boundary.contains_boid(&boid) {
                    return Err("Boundary doesn't contain boid");
                } else if boids.len() == MAX_BOID_IN_AREA {
                    log::debug!("to much boids in area. divide");
                    self.subdivide();
                    return self.insert(boid);
                } else {
                    log::info!("its ok. Adding boid!");
                    boids.push(boid);
                    return Ok(());
                }
            }
            QuadTree::Root { neighbours } => {
                log::info!("its root!");

                log::info!("loop over neighbours start");
                for n in neighbours {
                log::info!("loop over neighbours iterate");
                    let ok = n.insert(boid);
                    if ok.is_ok() {
                        return ok;
                    }
                }
                log::info!("loop over neighbours end");

                return Err("Boid couldn't be inserted in any sub-tree ");
            }
        }
    }

    fn subdivide(&mut self) {
        log::debug!("divide area");
        match self {
            QuadTree::Leaf {
                boundary,
                boids: points,
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
                    let _ = new.insert(*p);
                }
                let _ = mem::replace(self, new);
            }
            _ => {}
        }
    }
    pub fn get_all_boids_in_boundry(&self, query_boundry: &Region, found_boids: &mut Vec<Boid>) {
        match self {
            QuadTree::Leaf { boundary, boids } => {
                if !query_boundry.intersect_with(boundary) {
                    return;
                }
                for b in boids {
                    if query_boundry.contains_boid(b) {
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
#[test]
fn get_all_boids_in_boundry() {
    let r = Region::new(Vector2::new(0.0, 0.0), Vector2::new(300.0, 300.0));
    let amount = 10;
    let x = r.right_down.x / amount as f32;
    let y = r.right_down.y / amount as f32;
    let mut q = QuadTree::new(r.clone());

    for i in 0..amount {
        let _ = q.insert(Boid::new(
            Vector2::new(
                i as f32 * x + BOID_SIZE as f32,
                i as f32 * y + BOID_SIZE as f32,
            ),
            Vector2::zero(),
            Vector2::zero(),
        ));
    }

    assert_eq!(q.count(), amount);
    {
        let mut boids_in_region = vec![];
        q.get_all_boids_in_boundry(&r.clone(), &mut boids_in_region);
        assert_eq!(boids_in_region.len(), amount);
    }
    {
        for i in 0..amount {
            let r = Region::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(
                    i as f32 * x + BOID_SIZE as f32,
                    i as f32 * y + BOID_SIZE as f32,
                ),
            );
            let mut boids_in_region = vec![];
            q.get_all_boids_in_boundry(&r, &mut boids_in_region);
            assert_eq!(boids_in_region.len(), i + 1);
        }
    }
}

#[test]
fn get_all_boids_in_boundry_view_of_boid() {
    let r = Region::new(Vector2::new(0.0, 0.0), Vector2::new(300.0, 300.0));
    let amount = 10;
    let x = r.right_down.x / amount as f32;
    let y = r.right_down.y / amount as f32;
    let mut q = QuadTree::new(r.clone());
    let mut boids = vec![];

    for i in 0..3 {
        let boid = Boid::new(
            Vector2::new(
                i as f32 * x + BOID_SIZE as f32,
                i as f32 * y + BOID_SIZE as f32,
            ),
            Vector2::zero(),
            Vector2::zero(),
        );
        boids.push(boid);
        let _ = q.insert(boid);
    }
    {
        let distance = 1.0;
        for b in &boids {
            let r = Region::rect_from_center_with_distance(b.position, distance);
            let mut boids_in_region = vec![];
            q.get_all_boids_in_boundry(&r, &mut boids_in_region);
            assert_eq!(boids_in_region.len(), 1);
        }
    }
    {
        let distance = x;
        for b in &boids {
            let r = Region::rect_from_center_with_distance(b.position, distance);
            let mut boids_in_region = vec![];
            q.get_all_boids_in_boundry(&r, &mut boids_in_region);
            println!("{}", b.id);
            assert_eq!(boids_in_region.len(), 1); // its failing for somre reason MenosGrandes
        }
    }
}
