#![allow(non_upper_case_globals)]

use crate::*;



#[pyclass]
pub struct Constants {}

#[pymethods]
impl Constants {
    #[classattr]
    /// # Zero
    pub const ZERO: Scalar = Scalar::new(0.0);

    #[classattr]
    /// # Zero (vector)
    pub const NULL_VECTOR: Vector = Vector::new(Self::ZERO, Self::ZERO, Self::ZERO);
    
    #[classattr]
    /// # Origo
    /// The zero-point for the simulation
    pub const ORIGO: Point = Point::new(Self::ZERO, Self::ZERO, Self::ZERO);

    #[classattr]
    /// # Gravitational Constant
    /// Newton's gravitational constant: `6.6743 * 10^-11 m^3 kg^-1 s^-2`
    pub const G: Scalar = Scalar::new(6.6743e-11);
    
    #[classattr]
    /// # Gravitational Acceleration on Earth
    /// Value: `9.81 ms^-2`
    pub const g: Vector = Vector::new(Self::ZERO, Scalar::new(-9.81), Self::ZERO);

    #[classattr]
    /// # PI
    pub const PI: Scalar = Scalar::new(std::f64::consts::PI);

    #[classattr]
    /// # Astronomical Unit
    /// The average distance between the Earth and the Sun
    pub const AU: Scalar = Scalar::new(149_597_870_700.0);

    #[classattr]
    /// # Light Year
    /// The distance light travels in 365.25 days
    pub const LIGHTYEAR: Scalar = Scalar::new(9.4605284e15);

    #[classattr]
    /// # Earth's Mass
    /// The mass of Earth\
    /// Value: `5.97219 * 10^24 kg`
    pub const EARTH_MASS: Scalar = Scalar::new(5.97219e24);
    
    #[classattr]
    /// # Earth's Velocity
    /// The velocity of Earth\
    /// Value: `29 780 km/s` (pointing downwards)
    pub const EARTH_VELOCITY: Vector = Vector::new(Scalar::new(-29_780.0), Self::ZERO, Self::ZERO);

    #[classattr]
    /// # Solar Mass
    /// The mass of the Sun.\
    /// Value: `2 * 10^30 kg`
    pub const SOLAR_MASS: Scalar = Scalar::new(2e30);

    #[classattr]
    /// # Solar Luminosity
    /// The power emitted by the Sun.\
    /// Value: `3.828 * 10^26 W`
    pub const SOLAR_LUMINOSITY: Scalar = Scalar::new(3.828e26);

    #[classattr]
    /// # Stefan-Boltzmann Constant
    /// The Stefan-Boltzmann constant, for calculating radiation and luminosity from temperature\
    /// Value: `5.670367 * 10^-8 W m^-2 K^-4`
    pub const STEFAN_BOLTZMANN_CONSTANT: Scalar = Scalar::new(5.670367e-8);
    
    #[classattr]
    /// # One Second
    pub const SECOND: Scalar = Scalar::new(1.0);
    
    #[classattr]
    /// # One Minute
    pub const MINUTE: Scalar = Scalar::new(60.0);

    #[classattr]
    /// # One Hour
    pub const HOUR: Scalar = Scalar::new(3600.0);

    #[classattr]
    /// # One Day
    pub const DAY: Scalar = Scalar::new(86_400.0);

    #[classattr]
    /// # One Week
    pub const WEEK: Scalar = Scalar::new(604_800.0);

    #[classattr]
    /// # One Month
    pub const MONTH: Scalar = Scalar::new(2_419_200.0);

    #[classattr]
    /// # One Year
    pub const YEAR: Scalar = Scalar::new(31_556_926.0);
    
    #[classattr]
    /// # Lightspeed
    /// The speed of light moving in a vacuum\
    /// Value: `299 792 458 m/s`
    pub const c: Scalar = Scalar::new(299_792_458.0);
}