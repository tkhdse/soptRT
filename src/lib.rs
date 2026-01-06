mod parser;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod soptRT {
    use pyo3::prelude::*;
    use crate::parser;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a - b).to_string())
    }


    #[pyfunction]
    fn _parse_graph(graph: String) -> PyResult<String> {
        let result = parser::lower_fx_to_mlir(graph);
        Ok(result.unwrap_or("None".to_string()))
    }
}
