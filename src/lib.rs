mod point;
mod vector;
mod degree;
mod scalar;

use crate::{
    point::*,
    scalar::*,
    vector::*,
    degree::*
};

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule(name = "physics")]
fn physics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Scalar>()?;
    m.add_class::<Degree>()?;
    Ok(())
}
