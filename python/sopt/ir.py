import torch
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class Node:
    name: str
    op_name: str
    target: str
    args: List[str]

    # optional fields
    shape: Optional[List[int]] = None
    dtype: Optional[str] = None
    index: Optional[int] = None

def serialize_fx_to_json(graph: torch.fx.GraphModule):
    print(graph)
    
    nodes_data = []

    for index,node in enumerate(graph.nodes):
        val = node.meta.get('val')
        if node.op != "output":
            shape = list(val.shape)
            dtype = str(val.dtype)

            nodes_data.append(Node(
                node.name,
                node.op,
                str(node.target),
                [str(arg) for arg in node.args]
                shape,
                dtype,
                index
            ))
        else:
            nodes_data.append(Node(
                node.name,
                node.op,
                str(node.target),
                [str(arg) for arg in node.args]
                None,
                None,
                index
            ))

    return nodes_data