use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
mod sopt_rt {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[derive(Debug, FromPyObject)]
    pub struct Node {
        pub name: String,
        pub op_name: String,
        pub target: String,
        args: Vec<String>
    }

    #[pyfunction]
    fn compile(nodes: Vec<Node>) -> PyResult<i32> {
        let result = lower_fx_to_mlir(nodes)
            .map_err(|e| PyValueError::new_err(e))?;

        // call compile_graph()
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
    fn lower_fx_to_mlir(nodes: Vec<Node>) -> Result<i32, String> {
        for node in nodes {
            println!("Node: {}, Op: {}, Target: {}", node.name, node.op_name, node.target);
        }

        return Ok(0)
    }
}
