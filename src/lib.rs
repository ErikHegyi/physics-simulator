mod point;
mod vector;
mod degree;

use crate::{
    point::*,
    vector::*,
    degree::*
};

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn physics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Vector>()?;
    m.add_class::<Degree>()?;
    Ok(())
}
