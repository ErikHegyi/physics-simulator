mod point;
mod vector;
mod degree;
mod scalar;
mod radiation;
mod point_body;
mod constants;
mod astronomy;
mod graphics;

use crate::{
    constants::*,
    degree::*,
    point::*,
    point_body::*,
    radiation::*,
    scalar::*,
    vector::*,
};

use astronomy::astronomical_simulation::*;
use astronomy::planet::*;
use astronomy::star::*;
use graphics::window::*;



fn main() {
    let planets: Vec<Box<dyn Celestial + Sync + 'static>> = vec![
        Box::new(
            Star::new(
                String::from("Sun"),
                Vector::new(ZERO, ZERO, ZERO),
                Point::new(ZERO, ZERO, ZERO),
                Scalar::new(2e30), // Approximate mass of Sun in kg (1 SM ~ 1.989e30 kg)
                Scalar::new(696_340_000.0), // radius in km
            )
        ),

        // Mercury
        Box::new(Planet::new(
            String::from("Mercury"),
            Vector::new(
                ZERO,
                Scalar::new(47_400.0), // 47.4 km/s converted to m/s
                ZERO,
            ),
            Point::new(
                Scalar::new(58_000_000_000.0), // 58 million km to meters
                ZERO,
                ZERO,
            ),
            Scalar::new(3.3e22), // mass in kg
            Scalar::new(2_440_000.0), // radius in km
            PlanetType::Terrestrial,
        )),
    
        // Venus
        Box::new(Planet::new(
            String::from("Venus"),
            Vector::new(
                ZERO,
                Scalar::new(35_000.0), // 35 km/s to m/s
                ZERO,
            ),
            Point::new(
                ZERO,
                ZERO,
                Scalar::new(-108_000_000_000.0), // -108 million km to meters
            ),
            Scalar::new(4.87e24),
            Scalar::new(6_052_000.0),
            PlanetType::Terrestrial,
        )),
    
        // Earth
        Box::new(Planet::new(
            String::from("Earth"),
            Vector::new(
                Scalar::new(-29_780.0), // -29,780 m/s
                ZERO,
                ZERO,
            ),
            Point::new(
                ZERO,
                ZERO,
                AU, // 1 AU in meters (assuming you have this helper)
            ),
            Scalar::new(5.97e24),
            Scalar::new(6_378_000.0),
            PlanetType::Terrestrial,
        )),
    
        // Mars
        Box::new(Planet::new(
            String::from("Mars"),
            Vector::new(
                ZERO,
                ZERO,
                Scalar::new(-24_100.0), // -24.1 km/s to m/s
            ),
            Point::new(
                Scalar::new(228_000_000_000.0), // 228 million km to meters
                ZERO,
                ZERO,
            ),
            Scalar::new(6.42e23),
            Scalar::new(3_390_000.0),
            PlanetType::Terrestrial
            ))
    ]; 
    let mut sim = AstronomicalSimulation::new(
        scalar!(1), planets, String::from("Simulation"), 0.02, false
    );
    sim.run();
}