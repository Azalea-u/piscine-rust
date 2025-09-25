use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized + Copy
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 { type Item = u32; fn zero() -> u32 { 0 } fn one() -> u32 { 1 } }
impl Scalar for u64 { type Item = u64; fn zero() -> u64 { 0 } fn one() -> u64 { 1 } }
impl Scalar for i32 { type Item = i32; fn zero() -> i32 { 0 } fn one() -> i32 { 1 } }
impl Scalar for i64 { type Item = i64; fn zero() -> i64 { 0 } fn one() -> i64 { 1 } }
impl Scalar for f32 { type Item = f32; fn zero() -> f32 { 0.0 } fn one() -> f32 { 1.0 } }
impl Scalar for f64 { type Item = f64; fn zero() -> f64 { 0.0 } fn one() -> f64 { 1.0 } }

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Copy + PartialEq> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let mut result = self.0.clone();
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = result[i][j] + rhs.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar<Item = T> + Copy + PartialEq> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }
        let mut result = self.0.clone();
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = result[i][j] - rhs.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}
