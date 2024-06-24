use pyo3::prelude::*;
use clearscreen::clear;

pub mod input;
pub mod file;
pub mod constants;
pub mod choice;

#[pyfunction]
fn start() {
    //edit feature to be added after v1.0, maybe in v2.0
    clear().unwrap();
    choice::choice();
}

/// A Python module implemented in Rust
#[pymodule]
fn task_reminder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start, m)?)?;
    Ok(())
}