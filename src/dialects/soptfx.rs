use crate::compiler::FXNode;

// define opaque operators belonging to soptfx
// Build an Opaque Operation: 
/*  let op = OperationBuilder::new("torch.aten.add", location)
        .add_operands(&[lhs, rhs])
        .add_results(&[tensor_type])
        .build()
*/


pub fn build_soptfx_op(node: &FXNode) -> Result<i32, String> {
    let target = &node.target;

    // let op = OperationBuilder::new()
    Ok(0)
}