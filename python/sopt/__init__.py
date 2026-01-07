import torch
import torch.nn as nn
import sopt_rt
from .ir import serialize_fx_to_json

"""
fx graph:

%name = <op_name>[target=<target>](args=%arg1, %arg2, ...)

node.name 
node.op = op_name = placeholder | call_function | output | get_attr
node.target = ATen operator
node.args

return ... --> this is a node with op_name == output
"""


def compile(model: nn.Module):
    module = torch.export.export(model(), (torch.randn(100, 8), ))

    nodes_data = serialize_fx_to_json(module.run_decompositions().graph)
    rgraph = sopt_rt._parse_graph(nodes_data)
    print(rgraph)


