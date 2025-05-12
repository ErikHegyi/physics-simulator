use pyo3::prelude::*;
use crate::*;


#[pyclass]
#[derive(Clone, Debug)]
/// # Point Body
/// A body, with no volume, surface, ...
/// ## Attributes
/// `mass: Scalar`\
/// `velocity: Vector`\
/// `coordinates: Point`\
/// `charge: Scalar`
pub struct PointBody {
    #[pyo3(get, set)]
    pub mass: Scalar,
    
    #[pyo3(get, set)]
    pub velocity: Vector,
    
    #[pyo3(get, set)]
    pub coordinates: Point,
    
    #[pyo3(get, set)]
    pub charge: Scalar
}


#[pymethods]
impl PointBody {
    #[new]
    /// Create a new `PointBody` object
    pub fn new(mass: Scalar,
               velocity: Vector,
               coordinates: Point, 
               charge: Scalar) -> Self {
        Self { mass, velocity, coordinates, charge }
    }
    
    /// Calculate the momentum of the body
    pub fn momentum(&self) -> Vector {
        self.velocity.clone() * self.mass
    }
    
    /// Calculate the kinetic energy of the body
    pub fn kinetic_energy(&self) -> Scalar {
       scalar!(0.5) * self.mass * self.velocity.magnitude().pow(scalar!(2))
    }
    
    /// Calculate the potential energy of the body
    pub fn potential_energy(&self, height: Scalar, gravity: Vector) -> Scalar {
        self.mass * height * gravity.magnitude()
    }
    
    /// Calculate the acceleration of the body based on the force applied to it
    pub fn acceleration(&self, force: Vector) -> Vector {
        force / self.mass
    }
    
    /// Calculate the force applied based on the body's acceleration
    pub fn force(&self, acceleration: Vector) -> Vector {
        acceleration * self.mass
    }
    
    /// Calculate the distance between the body and a given point
    pub fn distance(&self, point: &Point) -> Scalar {
        self.coordinates.distance(point)
    }
    
    /// Calculate the gravitational force between two bodies
    pub fn gravitational_force(&self, other: Self) -> Vector {
        // Get the distance between the two bodies
        let distance: Scalar = self.distance(&other.coordinates);
        
        // Calculate the gravitational force between the two bodies
        let force: Scalar = Constants::G * self.mass * other.mass / distance.powi(2);
        
        // Calculate the vector from the magnitude and the direction
        Vector::from_magnitude(force, &other.coordinates, &self.coordinates)
    }
    
    /// Advance the body by `dt` seconds
    pub fn advance(&mut self, dt: Scalar) {
        self.coordinates.x += self.velocity.point.x * dt;
        self.coordinates.y += self.velocity.point.y * dt;
        self.coordinates.z += self.velocity.point.z * dt;
    }
    
    pub fn __repr__(&self) -> String { format!("{self:?}") }
}