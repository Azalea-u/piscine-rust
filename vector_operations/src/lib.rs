#[derive(Debug,Copy,Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i:T,
    pub j:T,
    pub k:T
}

use std::ops::{Add, Sub};

impl Add for ThreeDVector<i32> {
    type Output = ThreeDVector<i32>;
    fn add(self, other: Self) -> Self::Output {
        ThreeDVector {
            i: self.i + other.i, j: self.j + other.j, k: self.k + other.k 
        }
    }
}

impl Sub for ThreeDVector<i32> {
    type Output = ThreeDVector<i32>;
    fn sub(self, other: Self) -> Self::Output {
        ThreeDVector { 
            i: self.i - other.i, j: self.j - other.j, k: self.k - other.k 
        }
    }
}
