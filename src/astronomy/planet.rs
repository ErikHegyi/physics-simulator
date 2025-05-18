use std::fmt::{Display, Formatter};
use crate::*;


#[derive(Debug, Copy, Clone)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    Satellite
}


impl<P> From<P> for PlanetType where String: From<P> {
    fn from(value: P) -> Self {
        let string: String = String::from(value).to_lowercase().replace(' ', "");
        match string.as_str() {
            "terrestrial" => Self::Terrestrial,
            "gasgiant" => Self::GasGiant,
            "satellite" => Self::GasGiant,
            _ => panic!("Unknown planet type")
        }
    }
}


impl Display for PlanetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::Terrestrial => "Terrestrial",
            Self::GasGiant => "Gas Giant",
            Self::Satellite => "Satellite"
        })
    }
}

pub struct Planet {
    pub name: String,
    pub radius: Scalar,
    pub planet_type: PlanetType,
    point_body: PointBody
}

impl Planet {
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
    pub fn gravitational_force(&self, other: &PointBody) -> Vector { self.point_body.gravitational_force(other) }
    pub fn advance(&mut self, dt: Scalar) { self.point_body.advance(dt) }
    
    /* ----- PLANET METHODS ----- */
    pub fn surface_acceleration(&self) -> Vector {
        Vector::new(
            ZERO,
            -G * self.mass() / self.radius.powi(2),
            ZERO
        )
    }
}


impl Celestial for Planet {
    #[inline]
    fn point_body(&self) -> &PointBody { &self.point_body }
    #[inline]
    fn point_body_mut(&mut self) -> &mut PointBody { &mut self.point_body }
    #[inline]
    fn get_radius(&self) -> Scalar { self.radius }
    #[inline]
    fn get_name(&self) -> String { self.name.clone() }
    #[inline]
    fn is_star(&self) -> bool { false }
    #[inline]
    fn is_planet(&self) -> bool { true }
    #[inline]
    fn planet_type(&self) -> Option<PlanetType> { Some(self.planet_type) }
    #[inline]
    fn get_color(&self) -> [f64; 4] {
        match self.planet_type {
            PlanetType::Terrestrial => [0.2, 0.2, 0.2, 1.0],
            PlanetType::GasGiant => [0.5, 0.5, 0.8, 1.0],
            PlanetType::Satellite => [0.2, 0.2, 0.2, 1.0]
        }
    }
}


unsafe impl Sync for Planet {}



#[macro_export]
macro_rules! planet {
    (
        $name: expr,
        $velocity: tt,
        $coordinates: tt,
        $mass: expr,
        $radius: expr
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            point!($coordinates),
            scalar!($mass),
            scalar!($radius),
            PlanetType::Terrestrial
        )
    };
    (
        $name: expr,
        $velocity: expr,
        $coordinates: tt,
        $mass: expr,
        $radius: expr
    ) => {
        Planet::new(
            String::from($name),
            $velocity,
            point!($coordinates),
            scalar!($mass),
            scalar!($radius)
        )
    };
    (
        $name: expr,
        $velocity: tt,
        $coordinates: expr,
        $mass: expr,
        $radius: expr
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            $coordinates,
            scalar!($mass),
            scalar!($radius)
        )
    };
    (
        $name: expr,
        $velocity: expr,
        $coordinates: expr,
        $mass: expr,
        $radius: expr
    ) => {
        Planet::new(
            String::from($name),
            $velocity,
            $coordinates,
            scalar!($mass),
            scalar!($radius),
            PlanetType::Terrestrial
        )
    };
    (
        $name: expr,
        $velocity: tt,
        $coordinates: tt,
        $mass: expr,
        $radius: expr,
        $planet_type: literal
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            point!($coordinates),
            scalar!($mass),
            scalar!($radius),
            PlanetType::from($planet_type)
        )
    };
    (
        $name: expr,
        $velocity: tt,
        $coordinates: tt,
        $mass: expr,
        $radius: expr,
        $planet_type: expr
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            point!($coordinates),
            scalar!($mass),
            scalar!($radius),
            $planet_type
        )
    };
    (
        $name: expr,
        $velocity: expr,
        $coordinates: tt,
        $mass: expr,
        $radius: expr,
        $planet_type: literal
    ) => {
        Planet::new(
            String::from($name),
            $velocity,
            point!($coordinates),
            scalar!($mass),
            scalar!($radius),
            PlanetType::from($planet_type)
        )
    };
    (
        $name: expr,
        $velocity: expr,
        $coordinates: tt,
        $mass: expr,
        $radius: expr,
        $planet_type: expr
    ) => {
        Planet::new(
            String::from($name),
            $velocity,
            point!($coordinates),
            scalar!($mass),
            scalar!($radius),
            $planet_type
        )
    };
    (
        $name: expr,
        $velocity: tt,
        $coordinates: expr,
        $mass: expr,
        $radius: expr,
        $planet_type: literal
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            $coordinates,
            scalar!($mass),
            scalar!($radius),
            PlanetType::from($planet_type)
        )
    };
    (
        $name: expr,
        $velocity: tt,
        $coordinates: expr,
        $mass: expr,
        $radius: expr,
        $planet_type: expr
    ) => {
        Planet::new(
            String::from($name),
            vector!($velocity),
            $coordinates,
            scalar!($mass),
            scalar!($radius),
            $planet_type
        )
    };
}
