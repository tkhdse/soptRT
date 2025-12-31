# soptRT
sopt = "small optimizer"; a Python AI Optimizer and Code Generator.

Honestly, not fully sure what I want to do with this yet. I'm taking this opportunity to learn Rust though.
Get started with: 
```
cd backend
cargo run
```

soptRT aims to:
* Perform high-level deep learning optimizations
* Build automated code-generator with SOTA
This will mirror NVIDIA's TensorRT/nvFuser but for my local hardware (since NVIDIA GPU's are too expensive now). 

And I want to find ways to integrate LLM-specific optimzation as well. Will probably extend with TVM/MLIR/LLVM.

Backend will be written in Rust; I'm using this project as an opportunity to learn Rust. 

### tracer
This module captures the DFG from the Python/PyTorch code. The idea is to set up Rust bindings for PyTorch and trigger the compiler/optimizer with torch.compile().

### opt
tbd

### cgen
tbd

### backend
tbd