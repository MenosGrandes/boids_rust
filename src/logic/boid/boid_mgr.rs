use rayon::prelude::*;
use sdl2::render::WindowCanvas;

use crate::{
    camera::Camera,
    constants::{
        types::BoidId, DrawPrimitives, DRAW_PRIMITIVES, MAX_BOID_IN_AREA, SCREEN_SIZE,
        VIEW_PORT_SIZE,
    },
    graphics::renderer::Renderable,
    logic::behaviour::traits::{
        AlignBehaviour, Behaviour, BoundBehaviour, CohesionBehaviour, SeperateBehaviour,
    },
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{Magnitude, V2f32, Vector2},
    },
};

use super::{
    boid_impl::Boid,
    traits::{Updatable, UpdatableAcceleration},
};

pub struct BoidManager {
    pub boids: Vec<Boid>,
    pub behaviours: Vec<Box<dyn Behaviour>>,
    pub quad_tree: QuadTree,
    update_tick: u8,
}
impl BoidManager {
    pub fn new(starting_region: Region) -> Self {
        Self {
            boids: Vec::new(),
            behaviours: vec![
                Box::new(AlignBehaviour {}),
                Box::new(SeperateBehaviour {}),
                Box::new(CohesionBehaviour {}),
                Box::new(BoundBehaviour {}),
            ],
            quad_tree: QuadTree::new(starting_region),
            update_tick: 0,
        }
    }

    pub fn add_boid(&mut self, amount: u64) {
        for _i in 0..amount {
            let mut c = Vector2::random(-1.0, 1.0); //
            c.set_magnitude(2.0);
            self.boids.push(Boid::new(
                Vector2::random_from_vec(
                    Vector2::new(0.0, VIEW_PORT_SIZE.x as f32),
                    Vector2::new(0.0, VIEW_PORT_SIZE.y as f32),
                ),
                c,
            ));
        }
    }
    pub fn spawn_boid(&mut self, amount: u64) {
        self.boids = Vec::with_capacity(amount as usize);
        self.add_boid(amount);
    }
    pub fn remove_all_boids(&mut self) {
        self.boids = Vec::new();
    }

    fn update_boids_in_quad_tree_from_too(&mut self, boid_id: BoidId) {
        let mut other_visible_boids: Vec<Boid> = Vec::with_capacity(MAX_BOID_IN_AREA);
        let region: Region = Region::rect_from_center(self.boids[boid_id].position);
        self.quad_tree
            .get_all_boids_in_boundry(&region, &mut other_visible_boids);

        //There is only one boid in visible, same as the loop is in
        //No need to do anything.
        if other_visible_boids.len() == 1 {
            self.boids[boid_id].update(Vector2::zero());
            return;
        }

        for b in &other_visible_boids {
            /*
            for behaviour in &self.behaviours {
                self.boids[b.id].acceleration += behaviour.calculate(&b, &other_visible_boids);
            }*/
            let acceleration: V2f32 = self
                .behaviours
                .iter()
                .map(|behaviour| behaviour.calculate(&b, &other_visible_boids))
                .sum();
            self.boids[b.id].update(acceleration);
        }
    }
    fn update_boids_in_quad_tree(&mut self) {
        (0..self.boids.len()).into_iter().for_each(|boid_id| {
            self.update_boids_in_quad_tree_from_too(boid_id);
        });
    }
}
impl Renderable for BoidManager {
    fn render(&mut self, canvas: &mut WindowCanvas, camera: &Camera) {
        for b in self.boids.iter_mut() {
            b.render(canvas, camera);
        }

        DRAW_PRIMITIVES.with(|value| {
            if value.borrow().contains(DrawPrimitives::QUAD_TREE) {
                self.quad_tree.render(canvas, camera);
            }
        });

        DRAW_PRIMITIVES.with(|value| {
            if value.borrow().contains(DrawPrimitives::BOUND_VIEW) {
                let mut r: Region = Region::new(
                    Vector2::new(100.0, 100.0) - camera.pos,
                    Vector2::new(
                        (VIEW_PORT_SIZE.x - 100.0) as f32,
                        (VIEW_PORT_SIZE.y - 100.0) as f32,
                    ) - camera.pos,
                );
                r.render(canvas, camera);
            }
        });
    }
}

impl Default for BoidManager {
    fn default() -> Self {
        Self::new(Region::default())
    }
}

impl Updatable for BoidManager {
    fn update(&mut self) {
        if self.update_tick == crate::constants::UPDATE_EVERY_TICK {
            let scren_size_region: Region = Region::new(Vector2::new(0.0, 0.0), VIEW_PORT_SIZE);
            self.quad_tree = QuadTree::new(scren_size_region);
            for b in self.boids.iter_mut() {
                match self.quad_tree.insert(*b) {
                    Err(_err) => {
                        /*
                        log::error!(
                            "Panic {} for {:?}, quad_tree = {:?}",
                            err,
                            *b,
                            self.quad_tree
                        );
                        panic!("{}", err)
                        */
                    }
                    _ => {}
                }
            }
            self.update_tick = 0;
        }
        self.update_boids_in_quad_tree();
        self.update_tick += 1;
    }
}
#[test]
fn get_all_boids_in_boundry() {}

#[test]
fn update_boids_in_quad_tree() {}
