pub trait SubInto {
    fn sub_into(data: &Self) -> [Self; 4]
    where
        Self: Sized;
}
pub trait Intersect {
    fn intersect_with(&self, second: &Self) -> bool;
}
