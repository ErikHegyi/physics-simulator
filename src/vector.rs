use crate::*;


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
pub struct Vector {
    pub point: Point
}

impl Vector {
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { point: Point::new(x, y, z) }
    }

    /// Create a new vector from a point
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
    pub fn from_magnitude(magnitude: Scalar, a: &Point, b: &Point) -> Self {
        // Calculate the distances between the points
        let dx: Scalar = b.x - a.x;
        let dy: Scalar = b.y - a.y;
        let dz: Scalar = b.z - a.z;

        // Calculate the total distance
        let origo: Point = Point {
            x: Scalar::new(0.),
            y: Scalar::new(0.),
            z: Scalar::new(0.)
        };
        let distance: Scalar = Point::new(dx, dy, dz).distance(&origo);

        // Calculate one unit of x, y and z
        let unit_x: Scalar = dx / distance;
        let unit_y: Scalar = dy / distance; 
        let unit_z: Scalar = dz / distance;

        // Create a new vector
        Vector::new(
            -unit_x * magnitude,
            -unit_y * magnitude,
            -unit_z * magnitude,
        )
    }

    /// Calculate the magnitude of the vector
    pub fn magnitude(&self) -> Scalar {
        // Square of the magnitude in the horizontal plane
        let horizontal: f64 = self.point.x.value.powi(2) + self.point.z.value.powi(2);

        // Calculate the magnitude
        let magnitude: f64 = (horizontal + self.point.y.value.powi(2)).sqrt();

        Scalar::new(magnitude)
    }
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

impl std::ops::Mul<Scalar> for Vector {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self::Output {
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

impl std::ops::MulAssign<Scalar> for Vector {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.point.x *= rhs;
        self.point.y *= rhs;
        self.point.z *= rhs;
    }
}

impl std::ops::Div<Scalar> for Vector {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self::Output {
        Self::from_point(
            Point {
                x: self.point.x / rhs,
                y: self.point.y / rhs,
                z: self.point.z / rhs
            }
        )
    }
}

impl std::ops::DivAssign<Scalar> for Vector {
    fn div_assign(&mut self, rhs: Scalar) {
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
        write!(f, "{:e}", self.magnitude().value)
    }
}

impl std::fmt::UpperExp for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:E}", self.magnitude().value)
    }
}