use pyo3::prelude::*;

mod compiler;

/// A Python module implemented in Rust.
#[pymodule]
mod sopt_rt {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use crate::compiler;


    #[pyfunction]
    fn compile(nodes: Vec<compiler::FXNode>) -> PyResult<i32> {
        let graph = lower_fx_to_mlir(nodes)
            .map_err(|e| PyValueError::new_err(e))?;

        let result = compiler::compile_graph(graph)
            .map_err(|e| PyValueError::new_err(e))?;
        Ok(result)
    }

    // we can add more entrypoints here
    // e.g if we just want to run an optimization pass
    // ... 

    // map ATen Ops to MLIR Dialects:
    /* 
        e.g:    aten.add.Tensor -> arith.addf
                aten.mm         -> linalg.matmul
                aten.relu       -> arith.maxf
    */  
    fn lower_fx_to_mlir(nodes: Vec<compiler::FXNode>) -> Result<compiler::FXGraph, String> {
        
        for node in &nodes {
            println!("Node: {}, Op: {}, Target: {}", node.name, node.op_name, node.target);
        }

        return Ok(compiler::FXGraph{ nodes })
    }
}
