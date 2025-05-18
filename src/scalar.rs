use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};


/// # Scalar
/// A value with only magnitude and no direction
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar {
    pub value: f64
}

impl Scalar {
    pub const fn new(value: f64) -> Self {
        Self { value }
    }
    
    pub fn pow(&self, n: Self) -> Self { Self::new(self.value.powf(n.value)) }
    pub fn powi(&self, n: i32) -> Self { Self::new(self.value.powi(n)) }
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


#[macro_export]
macro_rules! scalar {
    () => { Scalar::new(0.0) };
    ($x: literal) => { Scalar::new(f64::from($x)) };
    ($x: ident) => { Scalar::new(f64::from($x)) };
    ($x: expr) => { Scalar::new(f64::from($x)) };
}