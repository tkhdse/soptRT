import torch
import torch.nn as nn

def compile(model: nn.Module):
    module = torch.export.export(model(), (torch.randn(100, 8), ))
    print(type(module.graph))
    print(module.graph)