use std::collections::VecDeque;

use super::{
    entity::{Entity, MAX_ENTITIES},
    signature::Signature,
};

pub struct EntityManager {
    avail_entities: VecDeque<Entity>,
    signatures: [Signature; MAX_ENTITIES as usize],
    living_count: usize,
}
impl EntityManager {
    pub fn new() -> Self {
        let e: VecDeque<Entity> = (0..=MAX_ENTITIES)
            .into_iter()
            .map(|id| id as Entity)
            .collect();
        let signatures = [(); MAX_ENTITIES as usize].map(|_| Signature::empty());
        EntityManager {
            living_count: usize::MIN,
            avail_entities: e,
            signatures,
        }
    }
    pub fn create_entity(&mut self) -> Result<Entity, &'static str> {
        self.living_count += 1;
        match self.avail_entities.pop_back() {
            Some(id) => Ok(id),
            None => Err("cannot create entity"),
        }
    }
}

#[test]
fn create_entity() {
    let mut em = EntityManager::new();
    let i = em.create_entity().unwrap();
    assert_eq!(i, MAX_ENTITIES as Entity - 1);
}
