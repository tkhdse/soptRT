import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
# from torch.fx import symbolic_trace, GraphModule

import sopt

@sopt.compile
class Net(nn.Module):
    def __init__(self):
        super().__init__()
        self.fc1 = nn.Linear(8, 16)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(16, 4)

    def forward(self, x):
        x = self.relu(self.fc1(x))
        return self.fc2(x)