use crate::scalar::Scalar;


/// A point in a 3-dimensional space
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: Scalar,  // Width
    pub y: Scalar,  // Height
    pub z: Scalar  // Depth
}

impl Point {
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    /// Calculate the distance between two points
    pub fn distance(&self, other: &Point) -> Scalar {
        let x: Scalar = other.x - self.x;
        let y: Scalar = other.y - self.y;
        let z: Scalar = other.z - self.z;

        Scalar::new(
            (x.value.powi(2) + y.value.powi(2) + z.value.powi(2)).sqrt()
        )
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}


#[macro_export]
macro_rules! point {
    () => { ORIGO };
    (O) => { ORIGO };
    (0) => { ORIGO };
    ($x: expr, $y: expr, $z: expr) => {
        Point::new(
            scalar!($x), scalar!($y), scalar!($z)
        )
    };
    (($x: expr, $y: expr, $z: expr)) => {
        Point::new(
            scalar!($x), scalar!($y), scalar!($z)
        )
    }
}