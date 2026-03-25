mod model;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A Python module implemented in Rust.
#[pymodule]
fn sumtree(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<model::SumTree>()?;
    m.add("__version__", crate::VERSION)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
