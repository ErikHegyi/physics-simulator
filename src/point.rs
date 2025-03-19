use pyo3::{pyclass, pymethods};

/// A point in a 3-dimensional space
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[pyclass]
pub struct Point {
    #[pyo3(get, set)]
    pub x: f64,  // Width
    #[pyo3(get, set)]
    pub y: f64,  // Height
    #[pyo3(get, set)]
    pub z: f64  // Depth
}

#[pymethods]
impl Point {
    #[new]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculate the distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        let x: f64 = other.x - self.x;
        let y: f64 = other.y - self.y;
        let z: f64 = other.z - self.z;

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    pub fn __add__(&self, rhs: Self) -> Self { self.clone() + rhs }
    pub fn __sub__(&self, rhs: Self) -> Self { self.clone() - rhs }
    pub fn __mul__(&self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
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