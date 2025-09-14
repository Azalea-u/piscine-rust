#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (x, y) if (x == self.r && y == self.g) || (x == self.g && y == self.r) => {
                std::mem::swap(&mut self.r, &mut self.g);
            }
            (x, y) if (x == self.r && y == self.b) || (x == self.b && y == self.r) => {
                std::mem::swap(&mut self.r, &mut self.b);
            }
            (x, y) if (x == self.r && y == self.a) || (x == self.a && y == self.r) => {
                std::mem::swap(&mut self.r, &mut self.a);
            }
            (x, y) if (x == self.g && y == self.b) || (x == self.b && y == self.g) => {
                std::mem::swap(&mut self.g, &mut self.b);
            }
            (x, y) if (x == self.g && y == self.a) || (x == self.a && y == self.g) => {
                std::mem::swap(&mut self.g, &mut self.a);
            }
            (x, y) if (x == self.b && y == self.a) || (x == self.a && y == self.b) => {
                std::mem::swap(&mut self.b, &mut self.a);
            }
            _ => {}
        }
        self
    }
}
