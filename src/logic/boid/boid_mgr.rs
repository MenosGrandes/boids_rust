use sdl2::render::WindowCanvas;

use crate::{
    constants::{types::AreaId, SCREEN_SIZE, VIEW_DISTANCE},
    graphics::renderer::Renderable,
    logic::behaviour::traits::{AlignBehaviour, Behaviour, CohesionBehaviour, SeperateBehaviour},
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{random_color, Magnitude, V2usize, Vector2},
    },
};

use super::{boid_impl::Boid, traits::Updatable};

#[derive(Debug)]
pub struct BoidInArea {
    pub boids: Vec<Boid>,
    pub area_id: AreaId,
}

impl BoidInArea {
    pub fn new(boids: Vec<Boid>, area_id: AreaId) -> Self {
        Self { boids, area_id }
    }
}

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
        for _ in 0..amount {
            let mut c = Vector2::random(-1.0, 1.0); //
            c.set_magnitude(2.0);
            self.boids.push(Boid::new(
                Vector2::random_from_vec(
                    Vector2::new(0.0, SCREEN_SIZE.x as f32),
                    Vector2::new(0.0, SCREEN_SIZE.y as f32),
                ),
                c,
                Vector2::new(0.01, 0.01),
                random_color(),
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
    pub fn update_boids_in_region(
        &self,
        quad_tree: &QuadTree,
        boids_to_return: &mut Vec<BoidInArea>,
    ) {
        match &quad_tree {
            QuadTree::Leaf {
                boundary: _,
                boids,
                id,
            } => {
                let b_in_area = BoidInArea::new(boids.to_vec(), *id);
                boids_to_return.push(b_in_area);
            }
            QuadTree::Root { neighbours } => {
                for n in neighbours {
                    self.update_boids_in_region(n, boids_to_return);
                }
            }
        }
    }

    fn update_boids_in_quad_tree(&mut self) {
        for current_boid_id in 0..self.boids.len() {
            let mut other_visible_boids: Vec<Boid> = Vec::new();
            let region: Region = Region::rect_from_center(self.boids[current_boid_id].position, VIEW_DISTANCE);
            self.quad_tree
                .get_all_boids_in_boundry(&region, &mut other_visible_boids);

            for b in &other_visible_boids {
                let mut b_copy = b.clone();
                for behaviour in &self.behaviours {
                    b_copy.acceleration += behaviour.calculate(&b_copy, &other_visible_boids);
                }
                b_copy.update();
                self.boids[b_copy.id as usize - 1] = b_copy;
            }
        }
    }
}
impl Renderable for BoidManager {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for b in self.boids.iter_mut() {
            b.render(canvas)?;
            let mut region: Region = Region::rect_from_center(b.position, VIEW_DISTANCE);
            region.render(canvas)?;
        }
        self.quad_tree.render(canvas)?;
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
            let mut len = 0;
            for b in self.boids.iter_mut() {
                let area_id = self.quad_tree.insert(*b, &mut len);
                match area_id {
                    Ok(id) => {
                        b.area_id = id;
                    }
                    Err(err) => panic!("{} {}", err, len),
                }
            }
            self.update_tick = 0;
        }
        self.update_boids_in_quad_tree();
        self.update_tick += 1;
    }
}
