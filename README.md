# soptRT
sopt = "small optimizer"; a Python AI Optimizer and Code Generator. This is my own stab at [graph compilers](https://github.com/Lightning-AI/lightning-thunder) for LLMs. I'm taking this opportunity to learn Rust/Mojo.
Get started with: 
```
cd backend
cargo run
```

High-level goals of soptRT:
* Perform high-level deep learning optimizations
* Build automated code-generator with SOTA
* Distributed-target support (figuring out details)
* Support for extensibility (CUDA/Triton/... kernels)
This will mirror NVIDIA's TensorRT/nvFuser but for my local hardware (since NVIDIA GPU's are too expensive now). 


And I want to find ways to integrate LLM-specific optimzation as well. Will probably extend with TVM/MLIR/LLVM.

## Setup
```
# melior/MLIR
brew install llvm@21

# optional: make a venv: 
# python3 -m venv venv && . bin/venv/activate
pip install -r requirements.txt
```

## Getting started
```
import sopt

@sopt.compile
def Model(nn.Module): 
#   model definition
```


### tracer
This module captures the DFG from the Python/PyTorch code using symbolic tracing with PyTorch FX (`torch.fx`). The idea is to set up Rust bindings for PyTorch (`melior`) and trigger the compiler/optimizer via a `compile()` call.

### opt
tbd

### cgen
tbd

### backend
tbd