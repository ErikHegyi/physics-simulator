mod point;
mod vector;
mod degree;
mod scalar;
mod radiation;
mod point_body;
mod constants;
mod star;
mod planet;

use crate::{
    point::*,
    scalar::*,
    vector::*,
    degree::*,
    radiation::*,
    point_body::*,
    star::*,
    planet::*,
    constants::*,
};

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule(name = "physics")]
fn physics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Scalar>()?;
    m.add_class::<Degree>()?;
    m.add_class::<Radiation>()?;
    m.add_class::<Constants>()?;
    m.add_class::<PointBody>()?;
    m.add_class::<Star>()?;
    m.add_class::<StarType>()?;
    m.add_class::<Planet>()?;
    m.add_class::<PlanetType>()?;
    Ok(())
}
