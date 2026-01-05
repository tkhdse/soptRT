import torch
import torch.nn as nn
import soptRT

def compile(model: nn.Module):
    module = torch.export.export(model(), (torch.randn(100, 8), ))

    rgraph = soptRT.parse_graph(str(module.run_decompositions().graph))
    print(rgraph)


