use std::{fmt, str::FromStr};

use crate::{
    logic::behaviour::traits::BorderBehaviourE,
    math::vec::{V2u32, Vector2},
};

pub const SCREEN_SIZE: V2u32 = Vector2::new(800, 600);
pub const BOID_SIZE: i16 = 1;
pub const VIEW_DISTANCE: f32 = 50.0;
pub static mut DRAW_VIEW: bool = false;
pub static mut BORDER_BEHAVIOUR: BorderBehaviourE = BorderBehaviourE::Reflect;
pub const MAX_BOID_SPEED: f32 = 5.0;
pub const MAX_BOID_FORCE: f32 = 0.2;

pub const ALLIGN_FACTOR: f32 = 0.5;
pub const COHESION_FACTOR: f32 = 0.2;
pub const SEPERATE_FACTOR: f32 = 0.5;

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct BehaviourEnabled: u32 {
        const  ALL_DISABLED = 0b00000000;
        const ALLIGN = 0b00000001;
        const COHESION = 0b00000010;
        const SEPERATE = 0b00000100;
    }
}

pub static mut BEHAVIOUR_ENABLED: BehaviourEnabled = BehaviourEnabled::ALL_DISABLED;

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
