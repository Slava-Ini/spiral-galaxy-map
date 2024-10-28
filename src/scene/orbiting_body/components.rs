use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct OrbitingBody {
    pub angle: f32,
    pub orbit_speed: f32,
    pub semi_axis: SemiAxis,
    pub y_rotation: Quat,
}

#[derive(Debug, Clone, Copy)]
pub struct SemiAxis {
    pub major: f32,
    pub minor: f32,
}

impl SemiAxis {
    fn get_scale(&self, new_major: f32) -> f32 {
        new_major / self.major
    }
}

impl Default for SemiAxis {
    fn default() -> Self {
        Self {
            major: 0.5,
            minor: 0.3,
        }
    }
}

impl Display for SemiAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "b: {:.2}, s: {:.2}", self.major, self.minor)
    }
}

impl Add<f32> for SemiAxis {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            major: self.major + rhs,
            minor: self.minor + self.get_scale(self.major) * rhs,
        }
    }
}

impl AddAssign<f32> for SemiAxis {
    fn add_assign(&mut self, rhs: f32) {
        self.major += rhs;
        self.minor += self.get_scale(self.major) * rhs;
    }
}

impl Sub<f32> for SemiAxis {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            major: self.major - rhs,
            minor: self.minor - self.get_scale(self.major) * rhs,
        }
    }
}

impl SubAssign<f32> for SemiAxis {
    fn sub_assign(&mut self, rhs: f32) {
        self.major -= rhs;
        self.minor -= self.get_scale(self.major) * rhs;
    }
}

impl Add for SemiAxis {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            major: self.major + rhs.major,
            minor: self.minor + rhs.minor,
        }
    }
}

impl Mul<f32> for SemiAxis {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            major: self.major * rhs,
            minor: self.minor * self.get_scale(self.major) * rhs,
        }
    }
}

impl Mul<SemiAxis> for f32 {
    type Output = SemiAxis;

    fn mul(self, rhs: SemiAxis) -> Self::Output {
        rhs * self
    }
}
