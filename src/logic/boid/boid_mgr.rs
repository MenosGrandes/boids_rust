use sdl2::render::WindowCanvas;

use crate::{
    constants::{types::AreaId, BOID_SIZE, SCREEN_SIZE},
    graphics::renderer::Renderable,
    logic::behaviour::traits::{
        AlignBehaviour, Behaviour, CohesionBehaviour, SeeBehaviour, SeperateBehaviour,
    },
    math::{
        quadtree::{quadt::QuadTree, region::Region},
        vec::{random_color, Magnitude, V2f32, V2usize, Vector2},
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

    pub fn add_boid(&mut self, amount: i16) {
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
    pub fn spawn_boid(&mut self, amount: i16) {
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
        let mut other_visible_boids: Vec<BoidInArea> = Vec::new();

        self.update_boids_in_region(&self.quad_tree, &mut other_visible_boids);

        for boids_and_area in other_visible_boids {

            for boid in &boids_and_area.boids {
                let mut b_copy = boid.clone();
                //let other_visible_boids = boid.get_other_visible(&boids_and_area.boids);
                for behaviour in &self.behaviours {
                    b_copy.acceleration += behaviour.calculate(&b_copy, &boids_and_area.boids);
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
            let r: Region = Region::new(
                V2usize::new(0, 0),
                V2usize::new(SCREEN_SIZE.x as usize, SCREEN_SIZE.y as usize),
            );
            self.quad_tree = QuadTree::new(r.clone());
            for b in self.boids.iter_mut() {
                let area_id = self.quad_tree.insert(*b);
                match area_id {
                    Ok(id) => {
                        b.area_id = id;
                    }
                    Err(err) => panic!("{}", err),
                }
            }
            self.update_tick = 0;
        }
        self.update_boids_in_quad_tree();
        self.update_tick += 1;
    }
}
