//! A number that carries, beside its value, how it changes with each of the
//! coordinates it was worked out from.
//!
//! One formula written over these gives both a place and the gradient of that
//! place, exactly: what a rule needs when the place it holds a point to is
//! itself worked out from other points, as a fillet's centre is from its
//! corner and its sides. Writing that derivative out by hand, through the
//! opening of the corner and the unit directions of its sides, is where a sign
//! goes missing; carried along with the value, it cannot drift from it.

use std::ops::{Add, Div, Mul, Neg, Sub};

use glam::DVec2;

/// A value and its derivative along each of `N` coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Jet<const N: usize> {
    pub value: f64,
    pub slope: [f64; N],
}

impl<const N: usize> Jet<N> {
    /// A value that moves with nothing.
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            slope: [0.0; N],
        }
    }

    /// The coordinate at `rank` itself.
    pub fn variable(value: f64, rank: usize) -> Self {
        let mut slope = [0.0; N];
        slope[rank] = 1.0;
        Self { value, slope }
    }

    pub fn sqrt(self) -> Self {
        let root = self.value.sqrt();
        self.through(root, 0.5 / root)
    }

    pub fn sin(self) -> Self {
        self.through(self.value.sin(), self.value.cos())
    }

    pub fn cos(self) -> Self {
        self.through(self.value.cos(), -self.value.sin())
    }

    pub fn abs(self) -> Self {
        match self.value < 0.0 {
            true => -self,
            false => self,
        }
    }

    /// The angle of the direction (`across`, `self`), as `f64::atan2` reads
    /// it with `self` as the height.
    pub fn atan2(self, across: Self) -> Self {
        let square = self.value * self.value + across.value * across.value;
        let mut slope = [0.0; N];
        for (rank, slope) in slope.iter_mut().enumerate() {
            *slope = (across.value * self.slope[rank] - self.value * across.slope[rank]) / square;
        }
        Self {
            value: self.value.atan2(across.value),
            slope,
        }
    }

    /// The value of `f(self)`, where `f` changes at `derivative` here.
    fn through(self, value: f64, derivative: f64) -> Self {
        Self {
            value,
            slope: self.slope.map(|slope| slope * derivative),
        }
    }
}

impl<const N: usize> Add for Jet<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut slope = self.slope;
        for (rank, slope) in slope.iter_mut().enumerate() {
            *slope += other.slope[rank];
        }
        Self {
            value: self.value + other.value,
            slope,
        }
    }
}

impl<const N: usize> Sub for Jet<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl<const N: usize> Neg for Jet<N> {
    type Output = Self;
    fn neg(self) -> Self {
        self.through(-self.value, -1.0)
    }
}

impl<const N: usize> Mul for Jet<N> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut slope = [0.0; N];
        for (rank, slope) in slope.iter_mut().enumerate() {
            *slope = self.slope[rank] * other.value + self.value * other.slope[rank];
        }
        Self {
            value: self.value * other.value,
            slope,
        }
    }
}

impl<const N: usize> Div for Jet<N> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut slope = [0.0; N];
        for (rank, slope) in slope.iter_mut().enumerate() {
            *slope = (self.slope[rank] * other.value - self.value * other.slope[rank])
                / (other.value * other.value);
        }
        Self {
            value: self.value / other.value,
            slope,
        }
    }
}

impl<const N: usize> Mul<f64> for Jet<N> {
    type Output = Self;
    fn mul(self, factor: f64) -> Self {
        self.through(self.value * factor, factor)
    }
}

impl<const N: usize> Add<f64> for Jet<N> {
    type Output = Self;
    fn add(self, offset: f64) -> Self {
        Self {
            value: self.value + offset,
            slope: self.slope,
        }
    }
}

/// A place whose two coordinates are each a [`Jet`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct JetPlace<const N: usize> {
    pub x: Jet<N>,
    pub y: Jet<N>,
}

impl<const N: usize> JetPlace<N> {
    /// A place that moves with nothing.
    #[cfg(test)]
    pub fn fixed(at: DVec2) -> Self {
        Self {
            x: Jet::constant(at.x),
            y: Jet::constant(at.y),
        }
    }

    /// A place free to move, its two coordinates at `rank` and `rank + 1`.
    pub fn free(at: DVec2, rank: usize) -> Self {
        Self {
            x: Jet::variable(at.x, rank),
            y: Jet::variable(at.y, rank + 1),
        }
    }

    #[cfg(test)]
    pub fn value(self) -> DVec2 {
        DVec2::new(self.x.value, self.y.value)
    }

    pub fn dot(self, other: Self) -> Jet<N> {
        self.x * other.x + self.y * other.y
    }

    /// The cross product's one component: positive when `other` turns
    /// counter-clockwise from `self`.
    pub fn perp_dot(self, other: Self) -> Jet<N> {
        self.x * other.y - self.y * other.x
    }

    pub fn length(self) -> Jet<N> {
        self.dot(self).sqrt()
    }

    /// The place a quarter turn counter-clockwise from this one.
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn scaled(self, by: Jet<N>) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
        }
    }

    pub fn normalized(self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }
}

impl<const N: usize> Add for JetPlace<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<const N: usize> Sub for JetPlace<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests;
