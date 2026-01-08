use crate::dialects::soptfx::build_soptfx_op;

use melior::{
    Context, 
    dialect::{DialectRegistry, arith, func}, 
    ir::{*, attribute::TypeAttribute}, 
    utility::register_all_dialects
};

use crate::sopt_rt::PyNode;
use pyo3::prelude::*;


pub struct FXNode {
    pub name: String,
    pub op_name: OpType,
    pub target: String,
    args: Vec<String>
}

// #[derive(Debug, FromPyObject)]
pub struct FXGraph {
    pub nodes: Vec<FXNode>
}

// FX Operation Types
pub enum OpType {
    Placeholder,
    CallFunction,
    Output,
    GetAttr
}

// Main pipeline
pub fn compile_graph(graph: FXGraph) -> Result<i32, String> {
    // init context
    let context = init_mlir_context();

    // build module
    let module = init_module(&context);

    // run conversion pass (to IR)
    // convert_to_soptfx();

    // run optimization passes
    // run converstion pass (to LLVM)
    // code generation

    Ok(0)
}

pub fn lower_fx_to_mlir(py_nodes: Vec<PyNode>) -> Result<FXGraph, String> {
        
    // for node in &nodes {
    //     println!("Node: {}, Op: {}, Target: {}", node.name, node.op_name, node.target);
    // }

    let nodes = py_nodes.into_iter()
        .map(|pynode| -> Result<FXNode, String> {
            Ok(FXNode {
                name: pynode.name,
                op_name: parse_op_type(&pynode.op_name)?,
                target: pynode.target,
                args: pynode.args
            })
        })
        .collect::<Result<Vec<FXNode>, String>>()?;

    return Ok(FXGraph{ nodes })
}

fn parse_op_type(op_name: &str) -> Result<OpType, String> {
    match op_name {
        "placeholder" => Ok(OpType::Placeholder),
        "call_function" => Ok(OpType::CallFunction),
        "output" => Ok(OpType::Output),
        "get_attr" => Ok(OpType::GetAttr),
        _ => Err(format!("Unknown op type: {}", op_name)),
    }
}


// Context setup
fn init_mlir_context() -> Context {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    
    // temporary 
    context.set_allow_unregistered_dialects(true);

    context
}

fn init_module(context: &Context) -> Module {
    let location = Location::unknown(context);
    let module = Module::new(location);
    module
}

fn convert_to_soptfx(graph: &FXGraph) -> Result<i32, String> {
    
    for node in &graph.nodes {
        // match node::op_name {
        //     OpType::Placeholder => ,
        //     OpType::CallFunction => ,
        //     OpType::Output => ,
        //     _ => 
        // }

        let op_res = build_soptfx_op(&node)
            .map_err(|e| format!("Could not convert operation '{}': {}", node.name, e))?;
    }

    Ok(0)
}