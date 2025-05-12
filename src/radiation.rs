use pyo3::prelude::*;
use crate::scalar::Scalar;


const LIGHTSPEED: Scalar = Scalar::new(299_792_458.0);


#[pyclass]
#[derive(Debug, Clone)]
pub struct Radiation {
    #[pyo3(get)]
    pub wavelength: Scalar,  // Unit: m

    #[pyo3(get)]  // Unit: s^-1
    pub frequency: Scalar,

    #[pyo3(get)]
    pub temperature: Scalar  // Unit: K
}


#[pymethods]
impl Radiation {
    #[new]
    pub fn new(temperature: Scalar) -> Self {
        // Calculate the wavelength
        // T = 2.9 * 10^6 / lambda (in nm) => lambda (in nm) = 2.9 * 10^6 / T
        let wavelength: Scalar = Scalar::new(2.9e-3) / temperature;

        // Calculate the frequency
        // lambda = c/f => f = c/lambda
        let frequency: Scalar = LIGHTSPEED / wavelength;

        Self { wavelength, frequency, temperature}
    }

    #[staticmethod]
    pub fn from_wavelength(wavelength: Scalar) -> Self {
        // Calculate the temperature
        // T = 2.9 * 10^6 / lambda (in nm)
        let temperature: Scalar = Scalar::new(2.9e-3) / wavelength;
        Self::new(temperature)
    }

    #[staticmethod]
    pub fn from_frequency(frequency: Scalar) -> Self {
        // Calculate the wavelength
        // lambda = c/f
        let wavelength: Scalar = LIGHTSPEED / frequency;
        Self::from_wavelength(wavelength)
    }
    
    #[staticmethod]
    pub fn color_from_wavelength(wavelength: Scalar) -> [f64; 4] {
        let wavelength: f64 = wavelength.value;
        match wavelength {
            ..380e-9 => {
                [1.0, 0.0, 1.0, 0.0]
            },
            380e-9..440e-9 => {
                [
                    -(wavelength - 440e-9) / 60e-9,
                    0.0,
                    1.0,
                    1.0
                ]
            },
            440.0e-9..490.0e-9 => {
                [
                    0.0,
                    (wavelength - 440e-9) / 50e-9,
                    1.0,
                    1.0
                ]
            },
            490.0e-9..510.0e-9 => {
                [
                    0.0,
                    1.0,
                    -(wavelength - 510e-9) / 20e-9,
                    1.0
                ]
            },
            510.0e-9..580.0e-9 => {
                [
                    (wavelength - 510e-9) / 70e-9,
                    1.0,
                    0.0,
                    1.0
                ]
            },
            580e-9..645e-9 => {
                [
                    1.0,
                    -(wavelength - 645e-9) / 65e-9,
                    0.0,
                    1.0
                ]
            },
            645.0e-9..780.0e-9 => {
                [
                    1.0,
                    0.0,
                    0.0,
                    1.0
                ]
            },
            _ => {
                [
                    1.0,
                    0.0,
                    0.0,
                    0.0
                ]
            }
        }
    }
    
    pub fn color(&self) -> [f64; 4] {
        Self::color_from_wavelength(self.wavelength)
    }
}