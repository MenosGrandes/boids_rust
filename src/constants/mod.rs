use std::{fmt, str::FromStr};

use crate::{
    logic::behaviour::traits::BorderBehaviourE,
    math::vec::{V2u32, Vector2},
};

pub const SCREEN_SIZE: V2u32 = Vector2::new(800, 600);
pub const BOID_SIZE: i16 = 4;
pub const VIEW_DISTANCE: f32 = BOID_SIZE as f32 * 10.0 as f32;

use std::cell::RefCell;
thread_local!(pub static DRAW_VIEW: RefCell<bool> = RefCell::new(false));

thread_local!(pub static BORDER_BEHAVIOUR: RefCell<BorderBehaviourE> = RefCell::new(BorderBehaviourE::GoThrough));

//pub static mut BORDER_BEHAVIOUR: BorderBehaviourE = BorderBehaviourE::Reflect;
pub const MAX_BOID_SPEED: f32 = 3.0;
pub const MAX_BOID_FORCE: f32 = 0.2;

pub const ALLIGN_FACTOR: f32 = 0.3;
pub const COHESION_FACTOR: f32 = 0.1;
pub const SEPERATE_FACTOR: f32 = 0.39;
pub const UPDATE_EVERY_TICK: u8 = 1;

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct BehaviourEnabled: u32 {
        const  ALL_DISABLED = 0b00000000;
        const ALLIGN = 0b00000001;
        const COHESION = 0b00000010;
        const SEPERATE = 0b00000100;
        const ALL_ENABLED = 0b00000111;
    }
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
    i: u64,
}

impl IdIterator {
    const MAX_VALUE: u64 = u64::MAX;
    pub fn get_next(&mut self) -> u64 {
        if self.i + 1 == Self::MAX_VALUE {
            self.i = 0;
        }
        self.i += 1;
        return self.i;
    }
    pub fn new() -> Self {
        Self { i: 0 }
    }
}
thread_local!(pub static AREA_ID_ITERATOR: RefCell<IdIterator> = RefCell::new(IdIterator::new()));
thread_local!(pub static BOID_ID_ITERATOR: RefCell<IdIterator> = RefCell::new(IdIterator::new()));

pub mod types;
