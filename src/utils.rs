use melior::{
    Context,
    ir::{
        Type,
        r#type::IntegerType
    }
};


pub fn map_dtype_to_mlir<'c>(ctx: &'c Context, dtype: &str) -> Type<'c> {
    // quantized ints -> 8-bit integer
    match dtype {
        "torch.float16" => Type::float16(ctx),
        "torch.float32" => Type::float32(ctx),
        "torch.float64" => Type::float64(ctx),
        "torch.int32"   => IntegerType::new(ctx, 32).into(),
        "torch.int64"   => IntegerType::new(ctx, 64).into(),
        "torch.bool"    => IntegerType::new(ctx, 1).into(),
        _ => panic!("Unsupported data type found: {}", dtype)
    }
}