use crate::*;


#[derive(Debug, Copy, Clone)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    Satellite
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