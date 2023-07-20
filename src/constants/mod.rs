use std::{fmt, str::FromStr};

use crate::{
    logic::behaviour::traits::BorderBehaviourE,
    math::{
        quadtree::region::Region,
        vec::{V2f32, V2u32, Vector2},
    },
};

pub const SCREEN_SIZE: V2u32 = Vector2::new(800, 600);
pub const VIEW_PORT_SIZE: V2f32 = Vector2::new(800.0, 600.0);
pub const BOID_SIZE: i16 = 1;
pub const VIEW_DISTANCE: f32 = BOID_SIZE as f32 * 10.0 as f32;

use std::cell::RefCell;

thread_local!(pub static BORDER_BEHAVIOUR: RefCell<BorderBehaviourE> = RefCell::new(BorderBehaviourE::GoThrough));
thread_local!(pub static CURRENT_VIEW_PORT: RefCell<Region> = RefCell::new(Region::new(Vector2::new(0.0,0.0),VIEW_PORT_SIZE )));

pub const MAX_BOID_SPEED: f32 = 6.1;
pub const MAX_BOID_FORCE: f32 = 0.501;
pub const UPDATE_EVERY_TICK: u8 = 1;
pub const BOIDS_AMOUNT: u64 = 2000;
pub const MAX_BOID_IN_AREA: usize = (BOIDS_AMOUNT as usize * 20) / 100 as usize + 1;

use bitflags::bitflags;
use sdl2::pixels::Color;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct BehaviourEnabled: u32 {
        const  ALL_DISABLED = 0b00000000;
        const ALLIGN = 0b00000001;
        const COHESION = 0b00000010;
        const SEPERATE = 0b00000100;
        const  BOUND= 0b00001000;
        const ALL_ENABLED = 0b00001111;
    }
}
pub struct BehaviourConsts;
impl BehaviourConsts {
    pub const ALLIGN_FACTOR: f32 = 0.3;
    pub const COHESION_FACTOR: f32 = 0.3;
    pub const SEPERATE_FACTOR: f32 = 0.3;
}

pub static mut BEHAVIOUR_ENABLED: BehaviourEnabled = BehaviourEnabled::ALL_ENABLED;

impl fmt::Display for BehaviourEnabled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for BehaviourEnabled {
    type Err = bitflags::parser::ParseError;

    fn from_str(behaviour_enabled: &str) -> Result<Self, Self::Err> {
        Ok(Self(behaviour_enabled.parse()?))
    }
}

pub struct IdIterator {
    i: usize,
}

impl IdIterator {
    const MAX_VALUE: usize = usize::MAX;
    pub fn get_next(&mut self) -> usize {
        if self.i + 1 == Self::MAX_VALUE {
            self.i = 0;
        }
        let id = self.i;
        self.i += 1;
        return id;
    }
    pub fn new() -> Self {
        Self { i: 0 }
    }
}
thread_local!(pub static BOID_ID_ITERATOR: RefCell<IdIterator> = RefCell::new(IdIterator::new()));

pub const BOID_COLOR: Color = Color::BLUE;
pub const REGION_COLOR: Color = Color::WHITE;
pub const VIEW_COLOR: Color = Color::RED;
pub const QUAD_TREE_COLOR: Color = Color::YELLOW;

pub mod types;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DrawPrimitives : u8{
        const  ALL_DISABLED = 0b000;
        const  QUAD_TREE = 0b001;
        const  BOID_VIEW = 0b010;
        const  BOUND_VIEW= 0b100;
        const ALL_ENABLED = 0b111;
    }
}
thread_local!(pub static DRAW_PRIMITIVES: RefCell<DrawPrimitives> = RefCell::new(DrawPrimitives::ALL_DISABLED));
