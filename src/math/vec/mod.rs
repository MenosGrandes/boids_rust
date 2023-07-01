use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::Rng;
use sdl2::pixels::Color;
use std::fmt;
use std::ops::{Add, Mul};
#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vector2<T>
where
    T: Default + Add + Mul,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Vector2<T> {
        Vector2 {
            x: Default::default(),
            y: Default::default(),
        }
    }
    /*
    pub fn dot(first: Vector2<T>,  second: Vector2<T>)
    {
        (first.x * second.x) + (first.y * second.y) as T

    }*/
}

pub fn random() -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2 {
        x: rng.gen_range(-1.0..1.0),
        y: rng.gen_range(-1.0..1.0),
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add + Copy,
{
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl<T> Mul<Vector2<T>> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add,
{
    type Output = Vector2<T>;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: Add<Output = T> + Default + Mul,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::new(x, y)
    }
}

impl<T> fmt::Display for Vector2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let c = Uniform::from(0..255);
    Color::RGB(c.sample(&mut rng), c.sample(&mut rng), c.sample(&mut rng))
}

/*
impl<T> Distribution<Point<T>> for Standard where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point<T>{
        Point {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}*/
impl Distribution<Vector2<i16>> for Uniform<i16> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2<i16> {
        let (rand_x, rand_y) = rng.gen();
        Vector2 {
            x: rand_x,
            y: rand_y,
        }
    }
}
