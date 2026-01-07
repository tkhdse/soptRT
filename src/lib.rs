use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
mod sopt_rt {
    use pyo3::prelude::*;

    #[derive(Debug, FromPyObject)]
    pub struct Node {
        pub name: String,
        pub op_name: String,
        pub target: String,
        args: Vec<String>
    }

    #[pyfunction]
    fn compile(nodes: Vec<Node>) -> PyResult<String> {
        let result = lower_fx_to_mlir(nodes);
        Ok(result.unwrap_or("None".to_string()))
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
    fn lower_fx_to_mlir(nodes: Vec<Node>) -> Option<String> {
        for node in nodes {
            println!("Node: {}, Op: {}, Target: {}", node.name, node.op_name, node.target);
        }
        return Some(String::from("Ok"))
    }
}
