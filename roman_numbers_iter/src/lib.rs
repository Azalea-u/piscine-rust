#[derive(Debug, PartialEq, Clone)]
pub enum RomanDigit {
    I, V, X, L, C, D, M,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl RomanNumber {
    pub fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();
        let values = [
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")
        ];

        for (value, symbols) in values {
            while num >= value {
                for c in symbols.chars() {
                    digits.push(match c {
                        'I' => RomanDigit::I,
                        'V' => RomanDigit::V,
                        'X' => RomanDigit::X,
                        'L' => RomanDigit::L,
                        'C' => RomanDigit::C,
                        'D' => RomanDigit::D,
                        'M' => RomanDigit::M,
                        _ => unreachable!(),
                    });
                }
                num -= value;
            }
        }

        RomanNumber(digits)
    }

    pub fn to_decimal(&self) -> u32 {
        let mut total = 0;
        let mut prev = 0;
        
        for digit in self.0.iter().rev() {
            let val = match digit {
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };
            total += if val < prev { -val } else { val };
            prev = val;
        }
        total as u32
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.to_decimal();
        *self = RomanNumber::from(current + 1);
        Some(self.clone())
    }
}
