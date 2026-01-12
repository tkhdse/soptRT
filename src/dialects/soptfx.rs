use crate::compiler::{FXNode, OpType};
use std::collections::HashMap;

use melior::{
    Context,
    ir::{
        Operation, 
        Value,
        Location, 
        Region, 
        Block,
        operation::{OperationBuilder}
    }
};

// define opaque operators belonging to soptfx

const DIALECT: &str = "soptfx";
type OpMap<'a> = &'a HashMap<&'a str, Value<'a,'a>>;


pub fn build_soptfx_op(ctx: &Context, node: &FXNode, value_map: &HashMap<&str, Value>) -> Result<i32, String> {
    // value_map  ->  [node.name, resultingOperation] (node.name gives us a unique identifier that we can reference given args)
    let op_id = &node.name;

    match &node.op_name {
        OpType::Placeholder => handle_placeholder_op(&value_map, op_id, node.index),
        OpType::CallFunction => {
            // println!("{:?}", node);
            let target = &node.target;
            let target_parts = target.split('.').collect::<Vec<&str>>();
            let node_type = format!("{}.{}_{}", DIALECT, target_parts[0], target_parts[1]);
            handle_callfunction_op(ctx, value_map, node, node_type)
        },
        OpType::Output => {
            handle_output_op(ctx, &value_map, node)
        },
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

fn handle_callfunction_op(ctx: &Context, value_map: OpMap, node: &FXNode, node_type: String) -> Result<i32, String> {
    // get operands (from map)
    let operand_values: Vec<Value> = node.args.iter()
        .map(|arg: &String| {
            value_map.get(arg.as_str())
                .cloned()
                .ok_or_else(|| format!("Node '{}' not found in value map", arg)) // returns Err
        })
        .collect::<Result<Vec<Value>, String>>()
        .map_err(|e| e)?;

    // construct operation -> build_op()
    let location = Location::unknown(ctx);
    let op = OperationBuilder::new(&node_type, location)
        .add_operands(&operand_values)
        // .add_results(&[tensor_type])
        .build();

    // save return in map
    Ok(0)
}

fn handle_output_op(ctx: &Context, value_map: OpMap, node: &FXNode) -> Result<i32, String>{
    // get operands (from map)
    let operand_values: Vec<Value> = node.args.iter()
        .map(|arg: &String| {
            value_map.get(arg.as_str())
                .cloned()
                .ok_or_else(|| format!("Node '{}' not found in value map", arg)) // returns Err
        })
        .collect::<Result<Vec<Value>, String>>()
        .map_err(|e| e)?;

    // construct operation -> build_op()
    let location = Location::unknown(ctx);
    let opname = format!("{}.return", DIALECT);
    let op = OperationBuilder::new(&opname, location)
        .add_operands(&operand_values)
        .build();

    // append to block

    Ok(0)
}