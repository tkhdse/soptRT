use crate::compiler::FXNode;
use std::collections::HashMap;

use melior::ir::{Operation, operation::{OperationBuilder}};

// define opaque operators belonging to soptfx

const DIALECT: &str = "soptfx";


pub fn build_soptfx_op(node: &FXNode, value_map: &HashMap<&str, Operation>) -> Result<i32, String> {
    let target = &node.target;
    // operator name: DIALECT + "." + ATEN_OP
    // current name example:    aten.permute.default    ->      soptfx.aten_permute
    //                          aten.relu.default       ->      soptfx.aten_relu

    // match node::op_name {
    //     OpType::Output => ,
    //     OpType::GetAttr => ,
    //     OpType::Placeholder => ,
    //     OpType::CallFunction => ,
    // }

    Ok(0)
}


fn handle_placeholder_op() -> Result<i32, String> {
    // get operands (from region/block)
    // save in map
    Ok(0)
}

fn handle_callfunction_op() -> Result<i32, String> {
    // get operands (from map)
    // construct operation -> build_op()
    // save return in map
    Ok(0)
}

fn handle_output_op() -> Result<i32, String>{
    // get operands (from map)
    // construct operation -> build_op()
    Ok(0)
}


fn build_op() -> Result<i32, String> {
    // let op = OperationBuilder::new("torch.aten.add", location)
    //     .add_operands(&[lhs, rhs])
    //     .add_results(&[tensor_type])
    //     .build();
    Ok(0)
}