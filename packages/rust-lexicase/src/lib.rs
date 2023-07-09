use pyo3::prelude::*;

#[pyclass]
#[derive(FromPyObject)]
pub struct PyshGpIndividual {
    pub _error_vector: Vec<f32>,
}

#[pyfunction]
fn select_one(pop: Vec<PyshGpIndividual>) -> PyResult<PyshGpIndividual> {
    Ok(pop.into_iter().next().unwrap())
}

#[pyfunction]
fn select(pop: Vec<PyshGpIndividual>, n: usize) -> PyResult<Vec<PyshGpIndividual>> {
    Ok(pop.into_iter().take(n).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_lexicase(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(select_one, m)?)?;
    m.add_function(wrap_pyfunction!(select, m)?)?;
    Ok(())
}

/*
class Individual:
     def __init__(self, errors):
             self._error_vector = errors
     def error_vector(self):
             return self._error_vector

winner = rust_lexicase.select_one([Individual([5, 8, 9]), Individual([3, 2, 0]), Individual([6, 3, 2])])

 */
