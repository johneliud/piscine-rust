#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if (self.r == first && self.g == second) || (self.r == second && self.g == first) {
            std::mem::swap(&mut self.r, &mut self.g);
        } else if (self.r == first && self.b == second) || (self.r == second && self.b == first) {
            std::mem::swap(&mut self.r, &mut self.b);
        } else if (self.r == first && self.a == second) || (self.r == second && self.a == first) {
            std::mem::swap(&mut self.r, &mut self.a);
        } else if (self.g == first && self.b == second) || (self.g == second && self.b == first) {
            std::mem::swap(&mut self.g, &mut self.b);
        } else if (self.g == first && self.a == second) || (self.g == second && self.a == first) {
            std::mem::swap(&mut self.g, &mut self.a);
        } else if (self.b == first && self.a == second) || (self.b == second && self.a == first) {
            std::mem::swap(&mut self.b, &mut self.a);
        }
        self
    }
}
