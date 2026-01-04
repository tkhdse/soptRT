import torch
import torch.nn as nn

def compile(model: nn.Module):
    module = torch.export.export(model(), (torch.randn(100, 8), ))
    print(module.graph)
    print('\n')
    print(module.run_decompositions().graph)