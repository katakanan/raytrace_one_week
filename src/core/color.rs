use image::Rgba;
use na::Vector3;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn zero() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn to_rgba(&self, gamma: f64) -> Rgba<u8> {
        Rgba([
            ((self.r as f64).powf(gamma) * 255.99) as u8,
            ((self.g as f64).powf(gamma) * 255.99) as u8,
            ((self.b as f64).powf(gamma) * 255.99) as u8,
            255,
        ])
    }

    pub fn from_vec(v: &Vector3<f64>) -> Color {
        Color::new(v.x, v.y, v.z)
    }

    pub fn to_vec(&self) -> Vector3<f64> {
        Vector3::new(self.r, self.g, self.b)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, d: f64) -> Color {
        Color {
            r: self.r * d,
            g: self.g * d,
            b: self.b * d,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, d: f64) -> Color {
        if d == 0.0 {
            panic!("cannot divide by zero.");
        }

        Color {
            r: self.r / d,
            g: self.g / d,
            b: self.b / d,
        }
    }
}
