import torch
from dataclasses import dataclass

@dataclass
class Node:
    name: str
    op_name: str
    target: str
    args: list

def serialize_fx_to_json(graph: torch.fx.GraphModule):
    nodes_data = []

    for node in graph.nodes:
        nodes_data.append(Node(
            node.name,
            node.op,
            str(node.target),
            [str(arg) for arg in node.args]
        ))

    return nodes_data