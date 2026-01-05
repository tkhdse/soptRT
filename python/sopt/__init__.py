import torch
import torch.nn as nn
import soptRT

def compile(model: nn.Module):
    module = torch.export.export(model(), (torch.randn(100, 8), ))
    print(module.graph)
    print('\n')
    print(module.run_decompositions().graph)

    print(soptRT.sum_as_string(20,5))

