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
        pub args: Vec<String>,

        // optional fields
        pub shape: Option<Vec<i64>>,
        pub dtype: Option<String>,
        pub index: Option<usize>,
    }


    #[pyfunction]
    fn compile(nodes: Vec<PyNode>) -> PyResult<i32> {
        let context = compiler::init_mlir_context();
        let sopt_compiler = compiler::SOPTCompiler{ ctx: context };
        let graph = sopt_compiler.lower_fx_to_mlir(nodes)
            .map_err(|e| PyValueError::new_err(e))?;

        let result = sopt_compiler.compile_graph(graph)
            .map_err(|e| PyValueError::new_err(e))?;
        Ok(result)
    }

    // we can add more entrypoints here
    // e.g if we just want to run an optimization pass
    // ... 
}
