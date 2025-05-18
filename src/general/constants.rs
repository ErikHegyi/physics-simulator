#![allow(non_upper_case_globals)]

use crate::*;


/// # Zero
pub const ZERO: Scalar = Scalar::new(0.0);


/// # Zero (vector)
pub const NULL_VECTOR: Vector = Vector::new(ZERO, ZERO, ZERO);


/// # Origo
/// The zero-point for the simulation
pub const ORIGO: Point = Point::new(ZERO, ZERO, ZERO);


/// # Gravitational Constant
/// Newton's gravitational constant: `6.6743 * 10^-11 m^3 kg^-1 s^-2`
pub const G: Scalar = Scalar::new(6.6743e-11);


/// # Gravitational Acceleration on Earth
/// Value: `9.81 ms^-2`
pub const g: Vector = Vector::new(ZERO, Scalar::new(-9.81), ZERO);


/// # PI
pub const PI: Scalar = Scalar::new(std::f64::consts::PI);


/// # Astronomical Unit
/// The average distance between the Earth and the Sun
pub const AU: Scalar = Scalar::new(149_597_870_700.0);


/// # Light Year
/// The distance light travels in 365.25 days
pub const LIGHTYEAR: Scalar = Scalar::new(9.4605284e15);


/// # Earth's Mass
/// The mass of Earth\
/// Value: `5.97219 * 10^24 kg`
pub const EARTH_MASS: Scalar = Scalar::new(5.97219e24);


/// # Earth's Velocity
/// The velocity of Earth\
/// Value: `29 780 km/s` (pointing downwards)
pub const EARTH_VELOCITY: Vector = Vector::new(Scalar::new(-29_780.0), ZERO, ZERO);


/// # Solar Mass
/// The mass of the Sun.\
/// Value: `2 * 10^30 kg`
pub const SOLAR_MASS: Scalar = Scalar::new(2e30);


/// # Solar Luminosity
/// The power emitted by the Sun.\
/// Value: `3.828 * 10^26 W`
pub const SOLAR_LUMINOSITY: Scalar = Scalar::new(3.828e26);


/// # Stefan-Boltzmann Constant
/// The Stefan-Boltzmann constant, for calculating radiation and luminosity from temperature\
/// Value: `5.670367 * 10^-8 W m^-2 K^-4`
pub const STEFAN_BOLTZMANN_CONSTANT: Scalar = Scalar::new(5.670367e-8);


/// # One Second
pub const SECOND: Scalar = Scalar::new(1.0);


/// # One Minute
pub const MINUTE: Scalar = Scalar::new(60.0);


/// # One Hour
pub const HOUR: Scalar = Scalar::new(3600.0);


/// # One Day
pub const DAY: Scalar = Scalar::new(86_400.0);


/// # One Week
pub const WEEK: Scalar = Scalar::new(604_800.0);


/// # One Month
pub const MONTH: Scalar = Scalar::new(2_419_200.0);


/// # One Year
pub const YEAR: Scalar = Scalar::new(31_556_926.0);


/// # Lightspeed
/// The speed of light moving in a vacuum\
/// Value: `299 792 458 m/s`
pub const c: Scalar = Scalar::new(299_792_458.0);
