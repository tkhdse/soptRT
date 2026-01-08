// Dialect representting soptfx post-fusion

// Rust conversion pass would look for "soptRT.aten_add" (Opaque) and replace with a Registered Op "arith.addf"

// Can we make this compiler run faster by immediately replacing all aten_add's from the FX graph with "arith.addf"? 