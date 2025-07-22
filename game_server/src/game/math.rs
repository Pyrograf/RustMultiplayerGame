use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};


const EQ_EPSILON: f32 = 0.001;

pub fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= EQ_EPSILON
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl Vec2F {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn get_length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn get_length(&self) -> f32 {
        self.get_length_squared().sqrt()
    }

    pub fn get_normal(&self) -> Self {
        let length = self.get_length();
        Vec2F {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn approx_eq(&self, v2: &Vec2F) -> bool {
        approx_eq(self.x, v2.x) && approx_eq(self.y, v2.y)
    }

    pub fn lerp(start: &Self, end: &Self, t: f32) -> Vec2F {
        Vec2F {
            x: lerp(start.x, end.x, t),
            y: lerp(start.y, end.y, t),
        }
    }
}

impl Add for Vec2F {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2F {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2F {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2F {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}

impl SubAssign for Vec2F {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec2F::new(3.0, 4.0);
        let v2 = Vec2F::new(2.0, 3.0);
        assert!((v1 + v2).approx_eq(&Vec2F::new(5.0, 7.0)));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec2F::new(3.0, 4.0);
        let v2 = Vec2F::new(2.0, 3.0);
        assert!((v1 - v2).approx_eq(&Vec2F::new(1.0, 1.0)));
    }

    #[test]
    fn test_normal() {
        let v1 = Vec2F::new(2.0, 0.0);
        assert!(v1.get_normal().approx_eq(&Vec2F::new(1.0, 0.0)));
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vec2F::new(1.0, 0.0);
        let scalar = 2.0;
        assert!((v1 * scalar).approx_eq(&Vec2F::new(2.0, 0.0)));
    }

    #[test]
    fn test_lerp() {
        assert!(Vec2F::lerp(
            &Vec2F::new(1.0, 0.0), 
            &Vec2F::new(2.0, 2.0), 
            0.5
        ).approx_eq(&Vec2F::new(1.5, 1.0)));

        assert!(Vec2F::lerp(
            &Vec2F::new(-1.0, -2.0),
            &Vec2F::new(1.0, 2.0),
            0.5
        ).approx_eq(&Vec2F::new(0.0, 0.0)));
    }
}
