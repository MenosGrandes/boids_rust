use sdl2::{pixels::Color, render::WindowCanvas};

use crate::{
    constants::{BOID_SIZE, SCREEN_SIZE, VIEW_DISTANCE},
    graphics::renderer::Renderable,
    logic::behaviour::traits::{AlignBehaviour, Behaviour, CohesionBehaviour, SeperateBehaviour},
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{random_color, Magnitude, V2usize, Vector2},
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
        let x: f32 = SCREEN_SIZE.x as f32 / amount as f32;
        let y: f32 = SCREEN_SIZE.y as f32 / amount as f32;
        for i in 0..amount {
            let mut c = Vector2::random(-1.0, 1.0); //
            c.set_magnitude(2.0);
            self.boids.push(Boid::new(
                /*
                Vector2::random_from_vec(
                    Vector2::new(0.0, SCREEN_SIZE.x as f32),
                    Vector2::new(0.0, SCREEN_SIZE.y as f32),
                )*/
                Vector2::new(
                    i as f32 * x + BOID_SIZE as f32,
                    i as f32 * y + BOID_SIZE as f32,
                ),
                c,
                Vector2::new(0.01, 0.01),
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

    fn update_boids_in_quad_tree(&mut self) {
        for boid_id in 0..self.boids.len() {
            log::info!("START loopin on boid {:?}", self.boids[boid_id]);
            let mut other_visible_boids: Vec<Boid> = Vec::new();
            let region: Region =
                Region::rect_from_center(self.boids[boid_id].position);
            self.quad_tree
                .get_all_boids_in_boundry(&region, &mut other_visible_boids);

            //There is only one boid in visible, same as the loop is in
            //No need to do anything.
            if other_visible_boids.len() == 1 {
                self.boids[boid_id].update();
                log::info!("END Only one. loopin on boid {:?}", self.boids[boid_id]);
                continue;
            }
            log::info!("Region{:?}", region);
            log::info!("Boids {:?}", other_visible_boids);

            for b in &other_visible_boids {
                log::info!("internal loop on boid {:?}", b);
                let mut b_copy = b.clone();
                for behaviour in &self.behaviours {
                    b_copy.acceleration += behaviour.calculate(&b_copy, &other_visible_boids);
                }
                if b_copy.acceleration != Vector2::zero() {
                    log::info!("Acceleration changed boid {:?}", b);
                }
                b_copy.update();
                self.boids[b_copy.id] = b_copy;
            }
            log::info!("END loopin on boid {:?}", self.boids[boid_id]);
        }
    }
}
impl Renderable for BoidManager {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        /*
        for b in self.boids.iter_mut() {
            b.render(canvas)?;
            let mut region: Region = Region::rect_from_center(b.position, VIEW_DISTANCE);
            region.render(canvas)?;
        }*/

        let mut r = Region::new(Vector2::new(0.0, 0.0), Vector2::new(300.0, 300.0));
        r.render(canvas)?;

        let mut q = QuadTree::new(r.clone());

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
        for mut b in boids {
            log::info!("START {}", b.id);
            let mut r = Region::rect_from_center(b.position);
            let mut boids_in_region = vec![];
            log::info!("query boundry {:?}", r);
            q.get_all_boids_in_boundry(&r, &mut boids_in_region);
            b.render(canvas)?;
            r.render(canvas)?;
            log::info!(" boids in region {:?}", boids_in_region);
            log::info!(" END{}", b.id);
        }
        q.render(canvas)?;

        //self.quad_tree.render(canvas)?;
        Ok(())
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
                let ok = self.quad_tree.insert(*b);
                match ok {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("{}", err);
                        panic!("{}", err)
                    }
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
