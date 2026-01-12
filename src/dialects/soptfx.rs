use crate::compiler::{FXNode, OpType};
use std::collections::HashMap;

use melior::{
    Context,
    ir::{
        Operation, 
        Location, 
        Region, 
        operation::{OperationBuilder}
    }
};

// define opaque operators belonging to soptfx

const DIALECT: &str = "soptfx";
type OpMap<'a> = &'a HashMap<&'a str, Operation<'a>>;


pub fn build_soptfx_op(ctx: &Context, node: &FXNode, value_map: &HashMap<&str, Operation>) -> Result<i32, String> {
    // value_map  ->  [node.name, resultingOperation] (node.name gives us a unique identifier that we can reference given args)
    let op_id = &node.name;

    match &node.op_name {
        OpType::Placeholder => handle_placeholder_op(&value_map, op_id, node.index),
        OpType::CallFunction => {
            // println!("{:?}", node);
            let target = &node.target;
            let target_parts = target.split('.').collect::<Vec<&str>>();
            let node_type = format!("{}.{}_{}", DIALECT, target_parts[0], target_parts[1]);
            handle_callfunction_op(ctx, value_map, node_type, op_id)
        },
        OpType::Output => handle_output_op(&value_map),
        _ => Err("GetAttr not yet supported.".to_string()) //OpType::GetAttr
    }
}


fn handle_placeholder_op(value_map: OpMap, op_id: &String, index: usize) -> Result<i32, String> {
    // get operands (from region/block)
    // let region = Region::new();
    // let block = region.append_block((Block::new(&[])));

    // let arg_value = block.argument(index).expect("Missing block argument");

    // save in map
    // value_map.insert(op_id, );
    Ok(0)
}

fn handle_callfunction_op(ctx: &Context, value_map: OpMap, node_type: String, op_id: &String) -> Result<i32, String> {
    // get operands (from map)
    // construct operation -> build_op()
    let result = build_op(ctx, node_type);
    // save return in map
    Ok(0)
}

fn handle_output_op(value_map: OpMap) -> Result<i32, String>{
    // get operands (from map)
    // construct operation -> build_op()
    Ok(0)
}


fn build_op(ctx: &Context, op_name: String) -> Result<i32, String> {
    let location = Location::unknown(ctx);
    let op = OperationBuilder::new(&op_name, location)
        // .add_operands(&[lhs, rhs])
        // .add_results(&[tensor_type])
        .build();
    Ok(0)
}