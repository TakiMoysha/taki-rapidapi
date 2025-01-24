use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass(frozen, module = "rapid_api.rapid_api")]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Klass {
    name: String,
    age: u8,
}

/// A Python module implemented in Rust.
#[pymodule]
fn rapid_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Klass>()?;
    Ok(())
}
