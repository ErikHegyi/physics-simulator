use pyo3::prelude::*;
use crate::*;


#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum StarType {
    O,  // Blue
    B,  // Blue - White
    A,  // White
    F,  // Yellow - White
    G,  // Yellow
    K,  // Orange
    M  // Red
}


#[pyclass]
#[derive(Clone, Debug)]
pub struct Star {
    #[pyo3(get, set)]
    pub name: String,
    
    #[pyo3(get, set)]
    pub radius: Scalar,
    
    #[pyo3(get, set)]
    pub radiation: Radiation,

    #[pyo3(get)]
    point_body: PointBody
}


#[pymethods]
impl Star {
    #[new]
    pub fn new(name: String,
               velocity: Vector,
               coordinates: Point,
               mass: Scalar,
               radius: Scalar) -> Self {
        let point_body: PointBody = PointBody::new(mass, velocity, coordinates, scalar!());
        let mut star: Self = Self {
            name,
            radius,
            radiation: Radiation::new(scalar!(0)),
            point_body
        };

        let surface_temp: Scalar = star.surface_temperature();
        star.radiation = Radiation::new(surface_temp);

        star
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

    /* ----- STAR METHODS ----- */
    pub fn surface_acceleration(&self) -> Vector {
        Vector::new(
            Constants::ZERO,
            -Constants::G * self.mass() / self.radius.powi(2),
            Constants::ZERO
        )
    }

    pub fn luminosity(&self) -> Scalar {
        Constants::SOLAR_LUMINOSITY * (self.mass() / Constants::SOLAR_MASS).pow(scalar!(3.5))
    }

    pub fn surface_temperature(&self) -> Scalar {
        let constant: Scalar = scalar!(4) * Constants::PI * Constants::STEFAN_BOLTZMANN_CONSTANT;
        (self.luminosity() / (constant * self.radius.powi(2))).pow(scalar!(0.25))
    }

    pub fn color(&self) -> [f64; 4] {
        match self.radiation.temperature.value {
            ..3500.0 => [1.0, 0.0, 0.0, 1.0],
            3500.0..6000.0 => [1.0, 1.0, 0.0, 1.0],
            6000.0..10000.0 => [1.0, 1.0, 1.0, 1.0],
            10000.0..25000.0 => [0.0, 1.0, 1.0, 1.0],
            25000.0.. => [0.35, 0.2, 0.35, 1.0],
            _ => [1.0, 0.0, 0.0, 1.0]
        }
    }

    pub fn star_type(&self) -> StarType {
        match self.radiation.temperature.value {
            ..3700.0 => { StarType::M },
            3700.0..5200.0 => { StarType::K },
            5200.0..6000.0 => { StarType::G },
            6000.0..7500.0 => { StarType::F },
            7500.0..10000.0 => { StarType::A },
            10000.0..30000.0 => { StarType::B },
            30000.0.. => { StarType::O },
            _ => { StarType::M }
        }
    }

    /* ----- PYTHON METHODS ----- */
    pub fn __repr__(&self) -> String { format!("Star {}", self.name) }
}