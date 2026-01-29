use crate::compiler::{FXNode, OpType};
use std::collections::HashMap;
use crate::utils::map_dtype_to_mlir;

use melior::{
    Context,
    ir::{
        Operation, 
        {operation::OperationLike},
        Value,
        Attribute,
        Location, 
        Region, 
        Block,
        r#type::RankedTensorType,
        operation::{OperationBuilder}
    }
};

// define opaque operators belonging to soptfx

const DIALECT: &str = "soptfx";

type OpMap<'a> = &'a mut HashMap<String, Value<'a,'a>>;


pub fn init_block(ctx: &Context) {
    // static region: Region = Region::new();
    // static block: Block = region.append_block((Block::new(&[])));

}

pub fn build_soptfx_op<'c>(ctx: &'c Context, node: &FXNode, value_map: &mut HashMap<String, Value<'c, 'c>>) -> Result<i32, String> {
    // value_map  ->  [node.name, resultingOperation] (node.name gives us a unique identifier that we can reference given args)
    let op_id = &node.name;

    match &node.op_name {
        OpType::Placeholder => handle_placeholder_op(value_map, op_id, node.index),
        OpType::CallFunction => {
            // println!("{:?}", node);
            let target = &node.target;
            let target_parts = target.split('.').collect::<Vec<&str>>();
            let node_type = format!("{}.{}_{}", DIALECT, target_parts[0], target_parts[1]);
            println!("name: {}, target: {}, node_type: {}", &node.name, &node.target, &node_type);
            handle_callfunction_op(ctx, value_map, node, node_type)
        },
        OpType::Output => {
            handle_output_op(ctx, value_map, node)
        },
        _ => Err("GetAttr not yet supported.".to_string()) //OpType::GetAttr
    }
}


fn handle_placeholder_op<'c>(value_map: &mut HashMap<String, Value<'c, 'c>>, op_id: &String, index: usize) -> Result<i32, String> {
    // get operands (from region/block)

    // let arg_value = block.argument(index).expect("Missing block argument");

    // save in map
    // value_map.insert(op_id, );
    Ok(0)
}

fn handle_callfunction_op<'c>(ctx: &'c Context, value_map: &mut HashMap<String, Value<'c, 'c>>, node: &FXNode, node_type: String) -> Result<i32, String> {
    // get operands (from map)
    let operand_values: Vec<Value> = node.args.iter()
        .map(|arg: &String| {
            value_map.get(arg)
                .cloned()
                .ok_or_else(|| format!("Node '{}' not found in value map", arg)) // returns Err
        })
        .collect::<Result<Vec<Value>, String>>()
        .map_err(|e| e)?;

    // construct operation -> build_op()
    let location = Location::unknown(ctx);

    if let Some(dtype_str) = &node.dtype {
        if let Some(shape) = &node.shape {

            // let shape = node.shape.as_ref()
            // .ok_or_else(|| format!("Node '{}' does not have shape metadata", node.name))?;

            // Convert i64 to u64
            let shape_u64: Vec<u64> = shape.iter()
                .map(|&dim| dim as u64)
                .collect();


            let dtype = map_dtype_to_mlir(ctx, dtype_str);
            let tensor_type = RankedTensorType::new(&shape_u64, dtype, None).into();
    
            let op = OperationBuilder::new(&node_type, location)
                .add_operands(&operand_values)
                .add_results(&[tensor_type])
                .build();
                // .map_err(|e| e.to_string())?;

            // save return in map
            let res = op.expect("Op produced no result");
            let value = res.result(0).expect("Op produced no result").into();
            value_map.insert(node.name.clone(), value);
        }
    } else {
        return Err(format!("Node {} does not have associated dtype", node.name))
    }

    Ok(0)
}

fn handle_output_op<'c>(ctx: &Context, value_map: &mut HashMap<String, Value<'c, 'c>>, node: &FXNode) -> Result<i32, String>{
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