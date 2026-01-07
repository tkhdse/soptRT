mod parser;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod sopt_rt {
    use pyo3::prelude::*;
    use crate::parser;

    #[pyfunction]
    fn _parse_graph(nodes: Vec<parser::Node>) -> PyResult<String> {
        let result = parser::lower_fx_to_mlir(nodes);
        Ok(result.unwrap_or("None".to_string()))
    }
}
