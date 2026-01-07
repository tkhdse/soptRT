use melior::{
    Context, 
    dialect::{DialectRegistry, arith, func}, 
    // ir::{*, attribute::TypeAttribute}, 
    utility::register_all_dialects
};

// Context setup
pub fn init_sopt() -> Context {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    context
}