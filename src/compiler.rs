use melior::{
    Context, 
    dialect::{DialectRegistry, arith, func}, 
    // ir::{*, attribute::TypeAttribute}, 
    utility::register_all_dialects
};

// Main pipeline
pub fn compile_graph(graph: FxGraph) -> Result<String> {
    let context = init_mlir_context();
    // build module
    // run conversion pass (to IR)
    // run optimization passes
    // run converstion pass (to LLVM)
    // code generation
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