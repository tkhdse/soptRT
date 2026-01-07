// this module will be used to parse the MLIR string fed in from PyTorch (decomposed Export IR)

// use melior::{
//     Context, dialect::{arith, func}, ir::{*, attribute::TypeAttribute}, 
//     utility::register_all_dialects
// };

use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
pub struct Node {
    pub name: String,
    pub op_name: String,
    pub target: String,
    args: Vec<String>
}


// map ATen Ops to MLIR Dialects:
/* 
    e.g:    aten.add.Tensor -> arith.addf
            aten.mm         -> linalg.matmul
            aten.relu       -> arith.maxf
*/  

pub fn lower_fx_to_mlir(nodes: Vec<Node>) -> Option<String> {
    for node in nodes {
        println!("Node: {}, Op: {}, Target: {}", node.name, node.op_name, node.target);
    }

    return Some(String::from("Ok"))
}