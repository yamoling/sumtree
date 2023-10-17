mod model;

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A Python module implemented in Rust.
#[pymodule]
fn sumtree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<model::SumTree>()?;
    m.add("__version__", crate::VERSION)?;
    Ok(())
}
