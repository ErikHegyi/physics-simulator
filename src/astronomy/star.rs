use crate::*;


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


#[derive(Clone, Debug)]
pub struct Star {
    pub name: String,
    pub radius: Scalar,
    pub radiation: Radiation,
    point_body: PointBody
}


impl Star {
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
    pub fn gravitational_force(&self, other: &PointBody) -> Vector { self.point_body.gravitational_force(other) }
    pub fn advance(&mut self, dt: Scalar) { self.point_body.advance(dt) }

    /* ----- STAR METHODS ----- */
    pub fn surface_acceleration(&self) -> Vector {
        Vector::new(
            ZERO,
            -G * self.mass() / self.radius.powi(2),
            ZERO
        )
    }

    pub fn luminosity(&self) -> Scalar {
        SOLAR_LUMINOSITY * (self.mass() / SOLAR_MASS).pow(scalar!(3.5))
    }

    pub fn surface_temperature(&self) -> Scalar {
        let constant: Scalar = scalar!(4) * PI * STEFAN_BOLTZMANN_CONSTANT;
        (self.luminosity() / (constant * self.radius.powi(2))).pow(scalar!(0.25))
    }

    pub fn color(&self) -> [f64; 4] {
        match self.radiation.temperature.value {
            ..3500.0 => [0.471, 0.035, 0.02, 1.0],
            3500.0..6000.0 => [1.0, 0.804, 0.0, 1.0],
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
}


impl Celestial for Star {
    #[inline]
    fn point_body(&self) -> &PointBody { &self.point_body }
    #[inline]
    fn point_body_mut(&mut self) -> &mut PointBody { &mut self.point_body }
    #[inline]
    fn get_radius(&self) -> Scalar { self.radius }
    #[inline]
    fn get_name(&self) -> String { self.name.clone() }
    #[inline]
    fn is_star(&self) -> bool { true }
    #[inline]
    fn is_planet(&self) -> bool { false }
    #[inline]
    fn planet_type(&self) -> Option<PlanetType> { None }
    #[inline]
    fn get_color(&self) -> [f64; 4] { self.color() }
}


#[macro_export]
macro_rules! star {
    (
        $name: expr,
        $velocity: tt,
        $coordinates: tt,
        $mass: expr,
        $radius: expr
    ) => {
        Star::new(
            String::from($name),
            vector!($velocity),
            point!($coordinates),
            scalar!($mass),
            scalar!($radius)
        )
    };
    (
        $name: expr,
        $velocity: expr,
        $coordinates: tt,
        $mass: expr,
        $radius: expr
    ) => {
        Star::new(
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
        Star::new(
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
        Star::new(String::from($name), $velocity, $coordinates, scalar!($mass), scalar!($radius))
    };
}
