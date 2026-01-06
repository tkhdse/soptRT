// this module will be used to parse the MLIR string fed in from PyTorch (decomposed Export IR)

use melior::{
    Context, dialect::{arith, func}, ir::{*, attribute::TypeAttribute}, 
    utility::register_all_dialects
};


// map ATen Ops to MLIR Dialects:
/* 
    e.g:    aten.add.Tensor -> arith.addf
            aten.mm         -> linalg.matmul
            aten.relu       -> arith.maxf
*/  

pub fn lower_fx_to_mlir(fx_graph: String) -> Option<String> {
    println!("lowered!");
    return Some("Ok".to_string())
}