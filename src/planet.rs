use crate::*;
use pyo3::prelude::*;


#[pyclass]
#[derive(Debug, Copy, Clone)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    Satellite
}


#[pymethods]
impl PlanetType {
    pub fn __repr__(&self) -> String {
        format!("{}", match *self {
            Self::Terrestrial => "Terrestrial Planet",
            Self::GasGiant => "Gas Giant",
            Self::Satellite => "Satellite"
        })
    }
}


#[pyclass]
pub struct Planet {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub radius: Scalar,

    #[pyo3(get, set)]
    pub planet_type: PlanetType,

    #[pyo3(get)]
    point_body: PointBody
}

#[pymethods]
impl Planet {
    #[new]
    pub fn new(name: String,
               velocity: Vector,
               coordinates: Point,
               mass: Scalar,
               radius: Scalar,
               planet_type: PlanetType) -> Self {
        let point_body: PointBody = PointBody::new(mass, velocity, coordinates, scalar!());
        Self {
            name,
            radius,
            planet_type,
            point_body
        }
    }
    

    /* ----- POINT BODY FIELDS ----- */
    pub fn mass(&self) -> Scalar { self.point_body.mass }
    pub fn velocity(&self) -> Vector { self.point_body.velocity.clone() }
    pub fn coordinates(&self) -> Point { self.point_body.coordinates.clone() }

    pub fn set_mass(&mut self, mass: Scalar) { self.point_body.mass = mass; }
    pub fn set_velocity(&mut self, velocity: Vector) { self.point_body.velocity = velocity; }
    pub fn set_coordinates(&mut self, coordinates: Point) { self.point_body.coordinates = coordinates; }

    /* ----- POINT BODY METHODS ----- */
    pub fn momentum(&self) -> Vector { self.point_body.momentum() }
    pub fn kinetic_energy(&self) -> Scalar { self.point_body.kinetic_energy() }
    pub fn acceleration(&self, force: Vector) -> Vector { self.point_body.acceleration(force) }
    pub fn force(&self, acceleration: Vector) -> Vector { self.point_body.force(acceleration) }
    pub fn distance(&self, other: &Point) -> Scalar { self.point_body.distance(other) }
    pub fn gravitational_force(&self, other: PointBody) -> Vector { self.point_body.gravitational_force(other) }
    pub fn advance(&mut self, dt: Scalar) { self.point_body.advance(dt) }
    
    /* ----- PLANET METHODS ----- */
    pub fn surface_acceleration(&self) -> Vector {
        Vector::new(
            Constants::ZERO,
            -Constants::G * self.mass() / self.radius.powi(2),
            Constants::ZERO
        )
    }

    /* ----- PYTHON METHODS ----- */
    pub fn __repr__(&self) -> String { format!("{} {}", self.planet_type.__repr__(), self.name) }
}