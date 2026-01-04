import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.fx import symbolic_trace, GraphModule

class Net(nn.Module):
    def __init__(self, input_size, num_classes):
        super().__init__()
        self.fc1 = nn.Linear(input_size, 16)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(16, num_classes)

    def forward(self, x):
        x = self.relu(self.fc1(x))
        return self.fc2(x)



net = Net(8, 2)
traced = symbolic_trace(net)
print(traced.graph)