use crate::{
    *
};


/// # Vector
/// A physical vector.
/// ## Attributes
/// The vector struct only has a `point` attribute.
/// This describes the end point of the vector, if the starting point is `[0, 0, 0]`.
/// ## Methods
/// The methods of the vector struct allow for
/// - Addition
/// - Subtraction
/// - Multiplication
#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[pyclass]
pub struct Vector {
    #[pyo3(get, set)]
    pub point: Point
}

#[pymethods]
impl Vector {
    #[new]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { point: Point::new(x, y, z) }
    }

    /// Create a new vector from a point
    #[staticmethod]
    pub fn from_point(point: Point) -> Self {
        Self { point }
    }

    /// Convert the `Vector` to a `Point`
    pub fn to_point(&self) -> Point {
        self.point
    }

    /// Create a new vector from a magnitude and a direction.
    /// # Params
    /// - `magnitude: Scalar` - An `f64`, signifying the calculated magnitude of the vector
    /// - `a: Point` - The point that the vector is pointing towards (the direction of the vector)
    /// - `b: Point` - The origin point of the vector
    #[staticmethod]
    pub fn from_magnitude(magnitude: f64, a: &Point, b: &Point) -> Self {
        // Calculate the distances between the points
        let dx: f64 = b.x - a.x;
        let dy: f64 = b.y - a.y;
        let dz: f64 = b.z - a.z;

        // Calculate the total distance
        let origo: Point = Point {
            x: 0.,
            y: 0.,
            z: 0.
        };
        let distance = Point::new(dx, dy, dz).distance(&origo);

        // Calculate one unit of x, y and z
        let unit_x = dx / distance;
        let unit_y = dy / distance;
        let unit_z = dz / distance;

        // Create a new vector
        Vector::new(
            -unit_x * magnitude,
            -unit_y * magnitude,
            -unit_z * magnitude,
        )
    }

    /// Calculate the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        // Square of the magnitude in the horizontal plane
        let horizontal: f64 = self.point.x.powi(2) + self.point.z.powi(2);

        // Calculate the magnitude
        let magnitude: f64 = (horizontal + self.point.y.powi(2)).sqrt();

        magnitude
    }

    pub fn __add__(&self, rhs: Self) -> Self { self.clone() + rhs }
    pub fn __sub__(&self, rhs: Self) -> Self { self.clone() - rhs }
    pub fn __mul__(&self, rhs: Self) -> Self { self.clone() * rhs }
    pub fn __truediv__(&self, rhs: f64) -> Self { self.clone() / rhs }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_point(self.point + rhs.point)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::from_point(self.point + rhs.point)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}


impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + -rhs
    }
}

impl std::ops::Mul<Vector> for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_point(
            Point {
                x: self.point.y * rhs.point.z - self.point.z * rhs.point.y,
                y: self.point.z * rhs.point.x - self.point.x * rhs.point.z,
                z: self.point.x * rhs.point.y - self.point.y * rhs.point.x
            }
        )
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_point(
            Point {
                x: self.point.x * rhs,
                y: self.point.y * rhs,
                z: self.point.z * rhs
            }
        )
    }
}

impl std::ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.point.x *= rhs;
        self.point.y *= rhs;
        self.point.z *= rhs;
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::from_point(
            Point {
                x: self.point.x / rhs,
                y: self.point.y / rhs,
                z: self.point.z / rhs
            }
        )
    }
}

impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.point.x /= rhs;
        self.point.y /= rhs;
        self.point.z /= rhs;
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            point: Point::new(-self.point.x, -self.point.y, -self.point.z)
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.magnitude())
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[0, 0, 0] -> [{:.5}, {:.5}, {:.5}]", self.point.x, self.point.y, self.point.z)
    }
}

impl std::fmt::LowerExp for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}", self.magnitude())
    }
}

impl std::fmt::UpperExp for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:E}", self.magnitude())
    }
}