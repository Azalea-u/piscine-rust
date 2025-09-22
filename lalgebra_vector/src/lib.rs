use std::ops::{Add, Mul};
use std::fmt::Debug;

pub trait Scalar:
    Add<Output = Self> + Mul<Output = Self> + Copy + Sized
{
}

impl Scalar for i64 {}
impl Scalar for i32 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for f32 {}
impl Scalar for f64 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = None;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let product = *a * *b;
            sum = match sum {
                Some(acc) => Some(acc + product),
                None => Some(product),
            };
        }
        sum
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let result = self
            .0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Some(Vector(result))
    }
}
