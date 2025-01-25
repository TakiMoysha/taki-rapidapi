use pyo3::{prelude::*, wrap_pymodule};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass(frozen, module = "_core._rs")]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Klass {
    name: String,
    age: u8,
}

/// A Python module implemented in Rust.
#[pymodule(module = "_core._rs")]
fn _rs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Klass>()?;
    Ok(())
}

#[pymodule(module = "rapid_api._core")]
fn rapid_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_wrapped(wrap_pymodule!(_rs))?;
    Ok(())
}
