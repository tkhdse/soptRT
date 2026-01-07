use melior::{
    Context, 
    dialect::{DialectRegistry, arith, func}, 
    // ir::{*, attribute::TypeAttribute}, 
    utility::register_all_dialects
};

use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
pub struct FXGraph {
    pub nodes: Vec<FXNode>
}

#[derive(Debug, FromPyObject)]
pub struct FXNode {
    pub name: String,
    pub op_name: String,
    pub target: String,
    args: Vec<String>
}

// Main pipeline
pub fn compile_graph(graph: FXGraph) -> Result<i32, String> {
    let context = init_mlir_context();
    // build module
    // run conversion pass (to IR)
    // run optimization passes
    // run converstion pass (to LLVM)
    // code generation

    Ok(0)
}


// Context setup
pub fn init_mlir_context() -> Context {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    context
}