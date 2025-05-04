use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use crate::*;


/// # Scalar
/// A value with only magnitude and no direction
#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[pyclass]
pub struct Scalar {
    #[pyo3(get, set)]
    pub value: f64
}

#[pymethods]
impl Scalar {
    #[new]
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn __add__(&self, rhs: Self) -> Self { Self::new(self.value + rhs.value) }
    pub fn __sub__(&self, rhs: Self) -> Self { Self::new(self.value - rhs.value) }
    pub fn __mul__(&self, rhs: Self) -> Self { Self::new(self.value * rhs.value) }
    pub fn __truediv__(&self, rhs: Self) -> Self { Self::new(self.value / rhs.value) }
}


impl Add for Scalar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value
        }
    }
}


impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}


impl Sub for Scalar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value
        }
    }
}


impl SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}


impl Mul for Scalar {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value
        }
    }
}


impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}


impl Div for Scalar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value / rhs.value
        }
    }
}


impl DivAssign for Scalar {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Neg for Scalar {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(
            -self.value
        )
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value >= 1e4 {
            write!(f, "{:e}", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}