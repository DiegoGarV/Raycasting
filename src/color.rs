use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(name: &str, red: u8, green: u8, blue: u8) -> Color {
        Color {
            name: String::from(name),
            red,
            green,
            blue,
        }
    }

    pub fn from_hex(name: &str, hex: u32) -> Color {
        let red = ((hex >> 16) & 0xFF) as u8;
        let green = ((hex >> 8) & 0xFF) as u8;
        let blue = (hex & 0xFF) as u8;

        Color::new(name, red, green, blue)
    }

    pub fn to_u32(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let red = (self.red as u16 + other.red as u16).min(255) as u8;
        let green = (self.green as u16 + other.green as u16).min(255) as u8;
        let blue = (self.blue as u16 + other.blue as u16).min(255) as u8;

        let name = format!("{} + {}", self.name, other.name);
        Color::new(&name, red, green, blue)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color {
        let clamped_factor = factor.max(0.0).min(1.0);

        let red = ((self.red as f32 * clamped_factor).min(255.0)).max(0.0) as u8;
        let green = ((self.green as f32 * clamped_factor).min(255.0)).max(0.0) as u8;
        let blue = ((self.blue as f32 * clamped_factor).min(255.0)).max(0.0) as u8;

        let name = format!("{} * {}", self.name, factor);
        Color::new(&name, red, green, blue)
    }
}