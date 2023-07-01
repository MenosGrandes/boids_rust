use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;
use sdl2::pixels::Color;
use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::Deref;
use std::ops::Div;
use std::ops::Sub;
use std::ops::{Add, Mul};



use num::NumCast;
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) { panic!(); }
    }
}
pub trait DotProduct<T = Self> {
    type Output;
    // add code here
    fn dot(self, other: Self) -> Self::Output;
}
pub trait Reflect<T> {
    fn reflect(self, other: Self) -> Self;
}
pub trait Distance<T> {
    fn distance(first: Self, other: Self) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> PartialEq for Vector2<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> DotProduct<Vector2<T>> for Vector2<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = T;
    fn dot(self, other: Self) -> T {
        return (self.x * other.x) + (self.y * other.y);
    }
}

impl<T> Vector2<T>
where
    T: Default + Add + Mul + Sub,
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
}

pub fn random(start: f32, end: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2 {
        x: rng.gen_range(start..end),
        y: rng.gen_range(start..end),
    }
}

impl<T> Sum<Vector2<T>> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add<Output = T> + Copy + Sub<Output = T>,
{
    fn sum<I: Iterator<Item = Vector2<T>>>(iter: I) -> Self {
        let mut ret: Vector2<T> = Vector2::zero();
        for v in iter {
            ret = ret + v;
        }
        ret
    }
}
impl<T> Sub<T> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add + Copy + Sub<Output = T>,
{
    type Output = Vector2<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add + Copy + Sub<Output = T>,
{
    type Output = Vector2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add + Copy + Sub,
{
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl<T> Mul<Vector2<T>> for Vector2<T>
where
    T: Mul<Output = T> + Default + Add + Sub,
{
    type Output = Vector2<T>;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}
impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: Add<Output = T> + Default + Mul + Sub,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self::new(x, y)
    }
}
impl<T> Div<Vector2<T>> for Vector2<T>
where
    T: Add<Output = T> + Default + Mul + Sub + Div<Output = T>,
{
    type Output = Vector2<T>;

    fn div(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}
impl<T> Div<T> for Vector2<T>
where
    T: Add<Output = T> + Default + Mul + Sub + Div<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
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

impl<T> Reflect<T> for Vector2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default + Mul<Output = T> + NumCast,
{
    fn reflect(self, other: Self) -> Self {
        self - (other * T::from(2).unwrap() * self.dot(other))
    }
}

impl<T> Distance<T> for Vector2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + Default + Mul<Output = T> + NumCast,
{
    fn distance(first: Self, other: Self) -> Self {
        first - other
    }
}

#[test]
fn new_test() {
    let v = Vector2::new(32, 28);
    assert_eq!(v.x, 32);
    assert_eq!(v.y, 28);
}
#[test]
fn eq_test() {
    let x = 11;
    let y = -3;
    let input = Vector2::new(x, y);
    let input_2 = Vector2::new(x, y);
    let input_reversed = Vector2::new(y, x);
    assert_eq!(input, input_2);
    assert_ne!(input, input_reversed);
}
#[test]
fn mul_vec_vec_test() {
    let x = 11;
    let y = -3;
    let input = Vector2::new(x, y);
    let input_2 = Vector2::new(y, x);
    let outcome = Vector2::new(x * y, y * x);
    let result = input * input_2;
    let result_reversed = input_2 * input;
    assert_eq!(result, outcome);
    assert_eq!(result_reversed, outcome);
}
#[test]
fn sub_vec() {
    let x = 11;
    let y = -3;
    let input = Vector2::new(x, y);
    let input_reversed = Vector2::new(y, x);
    {
        let result_vector = input - input_reversed;
        let result_vector_manual = Vector2::new(x - y, y - x);
        assert_eq!(result_vector, result_vector_manual);
    }
    {
        let result_vector = input_reversed - input;
        let result_vector_manual = Vector2::new(y - x, x - y);
        assert_eq!(result_vector, result_vector_manual);
    }
}
#[test]
fn mul_vec_scalar() {
    let x = 11;
    let y = -3;
    let scalar = 3;
    let input = Vector2::new(x, y);
    let result = input * scalar;
    let result_vector = Vector2::new(x * scalar, y * scalar);
    assert_eq!(result, result_vector);
}
#[test]
fn dot_product() {
    let x = 2;
    let y = 3;
    let input = Vector2::new(x, y);
    let dot = input.dot(input);
    assert_eq!(dot, 13);
}
#[test]
fn div_vec() {
    let x = 2.3;
    let y = 3.12;
    let input = Vector2::new(x, y);
    let input_2 = Vector2::new(x*2.0, y*2.0);
    let dot =  input / input_2;
    assert_eq!(dot, Vector2::new(0.5,0.5));
}
#[test]
fn distance() {
    let x = 2.3;
    let y = 3.12;
    let input = Vector2::new(x, y);
    let input_2 = Vector2::new(x, y);
    assert_eq!(Vector2::new(0.0,0.0), Vector2::distance(input,input_2));

}
#[test]
fn div_scalar() {
    let x = 2.3;
    let y = 3.12;
    let input = Vector2::new(x, y);
    let dot =  input / 2.0;
    assert_eq!(dot, Vector2::new(1.15,1.56));
}
