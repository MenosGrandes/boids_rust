use sdl2::render::WindowCanvas;

use crate::{
    constants::{BOID_SIZE, SCREEN_SIZE, MAX_BOID_IN_AREA},
    graphics::renderer::Renderable,
    logic::behaviour::traits::{AlignBehaviour, Behaviour, CohesionBehaviour, SeperateBehaviour},
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{DotProduct, Magnitude, V2f32, Vector2},
    },
};

use super::{boid_impl::Boid, traits::Updatable};

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
            ],
            quad_tree: QuadTree::new(starting_region),
            update_tick: 0,
        }
    }

    pub fn add_boid(&mut self, amount: u64) {
        /*
        let every = V2f32::new(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32) / amount as f32;
        for i in 0..amount {

            let mut c = Vector2::random(-1.0, 1.0); //
            c.set_magnitude(2.0);
            self.boids.push(Boid::new(
                    /*
                Vector2::random_from_vec(
                    Vector2::new(0.0, SCREEN_SIZE.x as f32),
                    Vector2::new(0.0, SCREEN_SIZE.y as f32),
                ),*/
                    Vector2::new(every.x + i as f32, every.y + i as f32),
                c,
                Vector2::new(0.01, 0.01),
            ) );
        }*/
        let area: f32 = SCREEN_SIZE.dot_self() as f32;
        let point_area = area / amount as f32;
        let length = point_area.sqrt() / 2.0;
        let mut i = length / 2.0 - BOID_SIZE as f32;
        let mut j = length / 2.0 - BOID_SIZE as f32;
        while i < SCREEN_SIZE.x as f32 - BOID_SIZE as f32 {
            while j < SCREEN_SIZE.y as f32 - BOID_SIZE as f32 {
                let mut c = Vector2::random(-1.0, 1.0); //
                c.set_magnitude(2.0);
                self.boids
                    .push(Boid::new(Vector2::new(i, j), c, Vector2::new(0.01, 0.01)));
                j += length;
            }
            i += length;
            j = length / 2.0 - BOID_SIZE as f32;
        }
    }
    pub fn spawn_boid(&mut self, amount: u64) {
        self.boids = Vec::with_capacity(amount as usize);
        self.add_boid(amount);
    }
    pub fn remove_all_boids(&mut self) {
        self.boids = Vec::new();
    }

    fn update_boids_in_quad_tree(&mut self) {
        for boid_id in 0..self.boids.len() {
            let mut other_visible_boids: Vec<Boid> = Vec::with_capacity(MAX_BOID_IN_AREA);
            let region: Region = Region::rect_from_center(self.boids[boid_id].position);
            self.quad_tree
                .get_all_boids_in_boundry(&region, &mut other_visible_boids);

            //There is only one boid in visible, same as the loop is in
            //No need to do anything.
            if other_visible_boids.len() == 1 {
                self.boids[boid_id].update();
                continue;
            }

            for b in &other_visible_boids {
                for behaviour in &self.behaviours {
                self.boids[b.id].acceleration +=behaviour.calculate(&b, &other_visible_boids);
                }
                self.boids[b.id].update();
            }
        }
    }
}
impl Renderable for BoidManager {
    fn render(&mut self, canvas: &mut WindowCanvas){
        for b in self.boids.iter_mut() {
            b.render(canvas);
        }

        self.quad_tree.render(canvas);
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
            let scren_size_region: Region = Region::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32),
            );
            self.quad_tree = QuadTree::new(scren_size_region);
            for b in self.boids.iter_mut() {
                match self.quad_tree.insert(*b) {
                    Err(err) => {
                        log::error!("Panic {} for {:?}, quad_tree = {:?}",err,*b, self.quad_tree);
                        panic!("{}", err)
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
