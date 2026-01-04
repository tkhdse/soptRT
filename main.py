import torch
from torch.fx import symbolic_trace, GraphModule


def my_func(x):
    return torch.relu(x).neg()

traced = symbolic_trace(my_func)
print(traced.graph)