#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![allow(clippy::redundant_pub_crate)]

use ec_core::{
    individual::Individual,
    operator::selector::{lexicase::Lexicase, Selector},
    test_results::{Error, TestResults},
};
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use rand::thread_rng;

struct PyIndividual {
    individual: PyObject,
    errors: TestResults<Error<i64>>,
}

impl PyIndividual {
    fn new(py: Python<'_>, individual: PyObject) -> PyResult<Self> {
        let errors = individual
            .getattr(py, "error_vector")? // Python-style getattr, requires a GIL token (`py`).
            .extract::<PyReadonlyArray1<i64>>(py)? // Tell PyO3 what to convert the result to.
            .as_array()
            .into_iter()
            .copied()
            .map(|v| Error { error: v })
            .collect::<TestResults<Error<i64>>>();
        Ok(Self { individual, errors })
    }

    fn py_individual(&self) -> PyObject {
        self.individual.clone()
    }
}

impl Individual for PyIndividual {
    type Genome = Self;

    type TestResults = TestResults<Error<i64>>;

    fn genome(&self) -> &Self::Genome {
        unimplemented!()
    }

    fn test_results(&self) -> &Self::TestResults {
        &self.errors
    }
}

#[pyfunction]
fn select_one(py: Python<'_>, pop: Vec<PyObject>) -> PyResult<PyObject> {
    let num_cases = PyIndividual::new(py, pop[0].clone())?
        .test_results()
        .results
        .len();
    let lexicase = Lexicase::new(num_cases);
    let population = pop
        .into_iter()
        .map(|i| PyIndividual::new(py, i))
        .collect::<PyResult<Vec<PyIndividual>>>()?;
    let mut rng = thread_rng();
    Ok(lexicase.select(&population, &mut rng)?.py_individual())
}

// #[pyfunction]
// fn select(pop: Vec<PyshGpIndividual>, n: usize) -> PyResult<Vec<PyshGpIndividual>> {
//     Ok(pop.into_iter().take(n).collect())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn rust_lexicase(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(select_one, m)?)?;
    // m.add_function(wrap_pyfunction!(select, m)?)?;
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
