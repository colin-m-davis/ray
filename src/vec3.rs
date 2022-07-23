use std::f64;
use std::fmt;
use std::ops::{ Add, Sub, Mul, Div };

// Vec3 definition
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Vec3 implementation
impl Vec3 {
    // Instance methods

    // Norm^2
    fn norm_squared(&self) -> f64 {
        Self::dot(&self, &self)
    }

    // Norm
    fn norm(&self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    // Static methods

    // Scalar multiplication
    pub fn smul(v: &Self, a: f64) -> Self {
        Self {
            x: a * v.x,
            y: a * v.y,
            z: a * v.z
        }
    }

    // Scalar division
    fn sdiv(v: &Self, a: f64) -> Self {
        Self {
            x: a / v.x,
            y: a / v.y,
            z: a / v.z
        }
    }

    // Additive inverse
    fn ainv(v: &Self) -> Self {
        Self {
            x: -v.x,
            y: -v.y,
            z: -v.z
        }
    }

    // Multiplicative inverse
    fn minv(v: &Self) -> Self {
        Self {
            x: 1. / v.x,
            y: 1. / v.y,
            z: 1. / v.z
        }
    }

    // Dot product
    fn dot(u: &Self, v: &Self) -> f64 {
        (u.x * v.x) + (u.y * v.y) + (u.z + v.z)
    }

    // Cross product
    fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    // Unit vector from vector
    fn unit(v: &Self) -> Self {
       Self::sdiv(v, v.norm())
    }

}

// Display implementation
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// Operator implementations

// +
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// -
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// *
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

// /
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

// Type aliases
pub type Point3 = Vec3;
pub type Color = Vec3;

// Wow, writing Rust code is a beautiful experience.