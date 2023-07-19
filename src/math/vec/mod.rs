use num::integer::Roots;

use num::Zero;
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
};
use sdl2::pixels::Color;
use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::NumCast;
pub trait DotProduct<T = Self> {
    type Output;
    fn dot(self, other: Self) -> Self::Output;
    fn dot_self(self) -> Self::Output;
}
pub trait Reflect<T> {
    fn reflect(self, other: Self) -> Self;
}
pub trait Normalize<T> {
    fn normalize(&mut self);
}
pub trait Distance<T = Self> {
    fn distance(first: Self, other: Self) -> Self;
}
pub trait InBetween<T = Self> {
    type Scalar;
    fn in_between(vector: Self, scalar: Self::Scalar) -> bool;
}

pub trait Magnitude<T> {
    type Output;
    fn calc_magnitude(&self) -> Self::Output;
    fn set_magnitude(&mut self, magnitude: T);
}

#[derive(Copy, Clone, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vector2<T>
where
    T: Default
        + Add
        + Mul
        + Sub
        + rand::distributions::uniform::SampleUniform
        + PartialOrd
        + Copy
        + DivAssign,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Vector2<T> {
        Vector2 {
            x: Default::default(),
            y: Default::default(),
        }
    }
    pub fn random(start: T, end: T) -> Vector2<T> {
        let mut rng = rand::thread_rng();
        Vector2 {
            x: rng.gen_range(start..end),
            y: rng.gen_range(start..end),
        }
    }
    pub fn random_from_vec(start: Vector2<T>, end: Vector2<T>) -> Vector2<T> {
        let mut rng = rand::thread_rng();
        Vector2 {
            x: rng.gen_range(start.x..start.y),
            y: rng.gen_range(end.x..end.y),
        }
    }
}

macro_rules! vect_impl {
    ($t:ty) => {
        impl PartialEq for Vector2<$t> {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl DotProduct<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t> + Add<Output = $t>,
        {
            type Output = $t;
            fn dot(self, other: Self) -> $t {
                return (self.x * other.x) + (self.y * other.y);
            }
            fn dot_self(self) -> $t {
                self.x * self.y
            }
        }

        impl Sum<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t>
                + Default
                + Add<Output = $t>
                + Copy
                + Sub<Output = $t>
                + SampleUniform
                + PartialOrd,
        {
            fn sum<I: Iterator<Item = Vector2<$t>>>(iter: I) -> Self {
                let mut ret: Vector2<$t> = Vector2::zero();
                for v in iter {
                    ret = ret + v;
                }
                ret
            }
        }
        //SubAssign

        impl SubAssign<$t> for Vector2<$t>
        where
            $t: Mul<Output = $t>
                + Default
                + Add
                + Copy
                + Sub<Output = $t>
                + SampleUniform
                + PartialOrd,
        {
            fn sub_assign(&mut self, rhs: $t) {
                *self = Self::new(self.x - rhs, self.y - rhs)
            }
        }

        impl SubAssign<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t>
                + Default
                + Add
                + Copy
                + Sub<Output = $t>
                + SampleUniform
                + PartialOrd,
        {
            fn sub_assign(&mut self, rhs: Vector2<$t>) {
                *self = Self::new(self.x - rhs.x, self.y - rhs.y)
            }
        }
        //END
        impl Sub<$t> for Vector2<$t>
        where
            $t: Mul<Output = $t>
                + Default
                + Add
                + Copy
                + Sub<Output = $t>
                + SampleUniform
                + PartialOrd,
        {
            type Output = Vector2<$t>;
            fn sub(self, rhs: $t) -> Self::Output {
                Self::new(self.x - rhs, self.y - rhs)
            }
        }

        impl Sub<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t>
                + Default
                + Add
                + Copy
                + Sub<Output = $t>
                + SampleUniform
                + PartialOrd,
        {
            type Output = Vector2<$t>;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Mul<$t> for Vector2<$t>
        where
            $t: Mul<Output = $t> + Default + Add + Copy + Sub + SampleUniform + PartialOrd,
        {
            type Output = Vector2<$t>;
            fn mul(self, rhs: $t) -> Self {
                Self::new(self.x * rhs, self.y * rhs)
            }
        }
        impl Mul<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t> + Default + Add + Sub + SampleUniform + PartialOrd + Copy,
        {
            type Output = Vector2<$t>;
            fn mul(self, rhs: Self) -> Self {
                Self::new(self.x * rhs.x, self.y * rhs.y)
            }
        }
        //MulAssign

        impl MulAssign<$t> for Vector2<$t>
        where
            $t: Mul<Output = $t> + Default + Add + Copy + Sub + SampleUniform + PartialOrd,
        {
            fn mul_assign(&mut self, rhs: $t) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }
        impl MulAssign<Vector2<$t>> for Vector2<$t>
        where
            $t: Mul<Output = $t> + Default + Add + Sub + SampleUniform + PartialOrd + Copy,
        {
            fn mul_assign(&mut self, rhs: Vector2<$t>) {
                self.x *= rhs.x;
                self.y *= rhs.y;
            }
        }
        //END
        //AddAssign

        impl AddAssign<Vector2<$t>> for Vector2<$t>
        where
            $t: Add<Output = $t> + Default + Mul + Sub + SampleUniform + PartialOrd + Copy,
        {
            fn add_assign(&mut self, rhs: Vector2<$t>) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }
        //END
        impl Add<Vector2<$t>> for Vector2<$t>
        where
            $t: Add<Output = $t> + Default + Mul + Sub + SampleUniform + PartialOrd + Copy,
        {
            type Output = Vector2<$t>;

            fn add(self, rhs: Vector2<$t>) -> Self::Output {
                let x = self.x + rhs.x;
                let y = self.y + rhs.y;
                Self::new(x, y)
            }
        }
        impl Add<$t> for Vector2<$t>
        where
            $t: Add<Output = $t> + Default + Mul + Sub + SampleUniform + PartialOrd + Copy,
        {
            type Output = Vector2<$t>;

            fn add(self, rhs: $t) -> Self::Output {
                let x = self.x + rhs;
                let y = self.y + rhs;
                Self::new(x, y)
            }
        }
        impl Div<Vector2<$t>> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Default
                + Mul
                + Sub
                + Div<Output = $t>
                + SampleUniform
                + PartialOrd
                + Copy,
        {
            type Output = Vector2<$t>;

            fn div(self, rhs: Vector2<$t>) -> Self::Output {
                Vector2::new(self.x / rhs.x, self.y / rhs.y)
            }
        }
        impl Div<$t> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Default
                + Mul
                + Sub
                + Div<Output = $t>
                + Copy
                + SampleUniform
                + PartialOrd,
        {
            type Output = Vector2<$t>;

            fn div(self, rhs: $t) -> Self::Output {
                Vector2::new(self.x / rhs, self.y / rhs)
            }
        }
        //DivAssign

        impl DivAssign<Vector2<$t>> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Default
                + Mul
                + Sub
                + Div<Output = $t>
                + SampleUniform
                + PartialOrd
                + Copy,
        {
            fn div_assign(&mut self, rhs: Vector2<$t>) {
                self.x /= rhs.x;
                self.y /= rhs.y;
            }
        }
        impl DivAssign<$t> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Default
                + Mul
                + Sub
                + Div<Output = $t>
                + Copy
                + SampleUniform
                + PartialOrd,
        {
            fn div_assign(&mut self, rhs: $t) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }
        //END

        impl fmt::Display for Vector2<$t>
        where
            $t: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl Reflect<$t> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Sub<Output = $t>
                + Copy
                + Default
                + Mul<Output = $t>
                + NumCast
                + SampleUniform
                + PartialOrd,
        {
            fn reflect(self, other: Self) -> Self {
                self - (other * <$t as NumCast>::from(2).unwrap() * self.dot(other))
            }
        }

        impl Distance<$t> for Vector2<$t>
        where
            $t: Add<Output = $t>
                + Sub<Output = $t>
                + Copy
                + Default
                + Mul<Output = $t>
                + NumCast
                + SampleUniform
                + PartialOrd,
        {
            fn distance(first: Self, other: Self) -> Self {
                first - other
            }
        }
        impl Normalize<$t> for Vector2<$t> {
            fn normalize(&mut self) {
                let c = self.calc_magnitude();
                if c > <$t>::zero() {
                    let out = Vector2::new(self.x / c, self.y / c);
                    self.x = out.x;
                    self.y = out.y;
                }
            }
        }
    };
}

macro_rules! set_magnitude_impl {
    ($t:ty) => {
        #[inline]
        fn set_magnitude(&mut self, magnitude: $t) {
            let current_magnitude = self.calc_magnitude();
            self.x = self.x * magnitude / current_magnitude;
            self.y = self.y * magnitude / current_magnitude;
        }
    };
}

macro_rules! magnitude_impl_int {
    ($t:ty) => {
        impl Magnitude<$t> for Vector2<$t> {
            type Output = $t;
            #[inline]
            fn calc_magnitude(&self) -> Self::Output {
                (self.x * self.x + self.y * self.y).sqrt()
            }
            set_magnitude_impl!($t);
        }
    };
}
macro_rules! magnitude_impl_float {
    ($t:ty) => {
        impl Magnitude<$t> for Vector2<$t> {
            type Output = $t;
            #[inline]
            fn calc_magnitude(&self) -> Self::Output {
                <$t>::sqrt(self.x * self.x + self.y * self.y)
            }
            set_magnitude_impl!($t);
        }
    };
}

macro_rules! in_between_impl {
    ($t:ty) => {
        impl InBetween<$t> for Vector2<$t> {
            type Scalar = $t;
            #[inline]
            fn in_between(vector: Self, scalar: Self::Scalar) -> bool {
                vector.x.abs() < scalar && vector.y.abs() < scalar
            }
        }
    };
}
magnitude_impl_float!(f32);
magnitude_impl_float!(f64);
magnitude_impl_int!(u8);
magnitude_impl_int!(u16);
magnitude_impl_int!(u32);
magnitude_impl_int!(usize);
magnitude_impl_int!(i32);
magnitude_impl_int!(i8);
magnitude_impl_int!(i16);
in_between_impl!(f32);
in_between_impl!(f64);
vect_impl!(f32);
vect_impl!(f64);
vect_impl!(u8);
vect_impl!(i8);
vect_impl!(u16);
vect_impl!(i16);
vect_impl!(i32);
vect_impl!(u32);
vect_impl!(usize);
pub type V2f32 = Vector2<f32>;
pub type V2f64 = Vector2<f64>;
pub type V2i32 = Vector2<i16>;
pub type V2u32 = Vector2<u32>;
pub type V2usize = Vector2<usize>;

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let c = Uniform::from(0..255);
    Color::RGB(c.sample(&mut rng), c.sample(&mut rng), c.sample(&mut rng))
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
    let input_2 = Vector2::new(x * 2.0, y * 2.0);
    let dot = input / input_2;
    assert_eq!(dot, Vector2::new(0.5, 0.5));
}
#[test]
fn distance() {
    let x = 2.3;
    let y = 3.12;
    let input = Vector2::new(x, y);
    let input_2 = Vector2::new(x, y);
    assert_eq!(Vector2::new(0.0, 0.0), Vector2::distance(input, input_2));
}
#[test]
fn div_scalar() {
    let x = 2.3;
    let y = 3.12;
    let input = Vector2::new(x, y);
    let dot = input / 2.0;
    assert_eq!(dot, Vector2::new(1.15, 1.56));
}
