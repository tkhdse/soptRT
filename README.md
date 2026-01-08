# soptRT
sopt = "small optimizer"; a PyTorch AI Optimizer and Code Generator. This is my own stab at [graph compilers](https://github.com/Lightning-AI/lightning-thunder) for LLMs. I'm taking this opportunity to learn Rust (`melior`/`PyO3`). Incrementally adding to [this dev blog](https://www.tkhadse.com/compiler) to discuss development and design choices.


High-level goals of soptRT:
* Perform high-level deep learning optimizations
* Build automated code-generator with SOTA
* Distributed-target support (figuring out details)
* Support for extensibility (CUDA/Triton/... kernels)
This will mirror NVIDIA's TensorRT/nvFuser but for my local hardware (since NVIDIA GPU's are too expensive now). 

To perform lowering, I utilize the following abstractions (from highest-levl to lowest-level):
* soptfx
* soptfuse
* linalg
* target

Additionally, I want to find ways to integrate LLM-specific optimzation as well. Will probably extend with TVM/MLIR/LLVM.

## Setup
```
# needed for melior
brew install llvm@21

# optional: make a venv: 
# python3 -m venv venv && . bin/venv/activate
pip install -r requirements.txt

# build Rust and Python import setup
maturin develop
```

## Getting started
```python
import torch
import torch.nn as nn
import sopt

@sopt.compile
class Model(nn.Module): 
#   model definition
```


### tracer
This module captures the DFG from the Python/PyTorch code using symbolic tracing with PyTorch FX (`torch.fx`). The idea is to set up Rust bindings for PyTorch (`PyO3`) and trigger the compiler/optimizer via the `@sopt.compile` decorator. `torch.fx` IR needs to be lowered to MLIR text for system compatibility.

### opt
tbd

### cgen
tbd

### backend
tbd