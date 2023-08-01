use super::component_type::MAX_COMPONENTS;

pub struct Signature {
    v: [bool; MAX_COMPONENTS as usize],
}

impl Signature {
    pub fn new(v: [bool; MAX_COMPONENTS as usize]) -> Self {
        Self { v }
    }
    pub fn empty() -> Self {
        Self {
            v: [false; MAX_COMPONENTS as usize],
        }
    }
}
