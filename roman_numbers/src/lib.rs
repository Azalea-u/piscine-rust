use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => unimplemented!(),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let mut digits = Vec::new();
        let mut n = n;

        if n >= 1000 {
            let q = n / 1000;
            n = n % 1000;
            for _ in 0..q {
                digits.push(M);
            }
        }

        if n >= 900 {
            digits.push(C);
            digits.push(M);
            n = n % 900;
        }

        if n >= 500 {
            digits.push(D);
            n = n % 500;
        }

        if n >= 400 {
            digits.push(C);
            digits.push(D);
            n = n % 400;
        }

        if n >= 100 {
            let q = n / 100;
            n = n % 100;
            for _ in 0..q {
                digits.push(C);
            }
        }

        if n >= 90 {
            digits.push(X);
            digits.push(C);
            n = n % 90;
        }

        if n >= 50 {
            digits.push(L);
            n = n % 50;
        }

        if n >= 40 {
            digits.push(X);
            digits.push(L);
            n = n % 40;
        }

        if n >= 10 {
            let q = n / 10;
            n = n % 10;
            for _ in 0..q {
                digits.push(X);
            }
        }

        if n >= 9 {
            digits.push(I);
            digits.push(X);
            n = n % 9;
        }

        if n >= 5 {
            digits.push(V);
            n = n % 5;
        }

        if n >= 4 {
            digits.push(I);
            digits.push(V);
            n = n % 4;
        }

        for _ in 0..n {
            digits.push(I);
        }

        if digits.is_empty() {
            digits.push(Nulla);
        }

        RomanNumber(digits)
    }
}
