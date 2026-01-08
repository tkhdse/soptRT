// define opaque operators belonging to soptfx

// aten operators
struct ATenNode {

}


/* 
    Build an Opaque Operation: 
    let op = OperationBuilder::new("torch.aten.add", location)
        .add_operands(&[lhs, rhs])
        .add_results(&[tensor_type])
        .build()
*/