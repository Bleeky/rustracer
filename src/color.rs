use std::ops::{Add, Mul, Sub};
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn black() -> Self {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn white() -> Self {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn gray() -> Self {
        Color {
            red: 0.1,
            green: 0.1,
            blue: 0.1,
        }
    }

    pub fn braun() -> Self {
        Color {
            red: 1.0,
            green: 0.5,
            blue: 0.5,
        }
    }

    pub fn yellow() -> Self {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
        }
    }

    pub fn orange() -> Self {
        Color {
            red: 1.0,
            green: 0.67,
            blue: 0.1,
        }
    }

    pub fn green() -> Self {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
        }
    }

    pub fn purple() -> Self {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 1.0,
        }
    }

    pub fn cyan() -> Self {
        Color {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn red() -> Self {
        Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::white()
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Sub<f32> for Color {
    type Output = Color;

    fn sub(self, other: f32) -> Color {
        Color {
            red: self.red - other,
            blue: self.blue - other,
            green: self.green - other,
        }
    }
}

impl Add<f32> for Color {
    type Output = Color;

    fn add(self, other: f32) -> Color {
        Color {
            red: self.red + other,
            blue: self.blue + other,
            green: self.green + other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            blue: self.blue - other.blue,
            green: self.green - other.green,
        }
    }
}
