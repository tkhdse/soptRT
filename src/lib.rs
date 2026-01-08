use pyo3::prelude::*;

mod compiler;
mod dialects;

/// A Python module implemented in Rust.
#[pymodule]
mod sopt_rt {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use crate::compiler;


    #[derive(Debug, FromPyObject)]
    pub struct PyNode {
        pub name: String,
        pub op_name: String,
        pub target: String,
        pub args: Vec<String>
    }


    #[pyfunction]
    fn compile(nodes: Vec<PyNode>) -> PyResult<i32> {
        let graph = compiler::lower_fx_to_mlir(nodes)
            .map_err(|e| PyValueError::new_err(e))?;

        let result = compiler::compile_graph(graph)
            .map_err(|e| PyValueError::new_err(e))?;
        Ok(result)
    }

    // we can add more entrypoints here
    // e.g if we just want to run an optimization pass
    // ... 
}
