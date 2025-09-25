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

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Copy + PartialEq> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() { 0 } else { self.0[0].len() }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        (0..self.number_of_rows()).map(|i| self.0[i][n]).collect()
    }
}

impl<T: Scalar<Item = T> + Copy + PartialEq> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::zero(); other.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j] + self.0[i][k] * other.0[k][j];
                }
            }
        }

        Some(Matrix(result))
    }
}
