import torch.nn as nn
import torch


class Network_10_type1(nn.Module):
    def __init__(self):
        super(Network_10_type1, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(10, 10),
            nn.ReLU(),
            nn.Linear(10, 10),
            nn.ReLU(),
            nn.Linear(10, 1),
        )

    def forward(self, x: torch.Tensor):
        return self.fc(x)


class Network_10_type2(nn.Module):
    def __init__(self):
        super(Network_10_type2, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(10, 20),
            nn.ReLU(),
            nn.Linear(20, 10),
            nn.ReLU(),
            nn.Linear(10, 1),
        )

    def forward(self, x: torch.Tensor):
        return self.fc(x)


class Network_8_type1(nn.Module):
    def __init__(self):
        super(Network_8_type1, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(8, 8),
            nn.ReLU(),
            nn.Linear(8, 8),
            nn.ReLU(),
            nn.Linear(8, 1),
        )

    def forward(self, x: torch.Tensor):
        return self.fc(x)


class Network_1_type1(nn.Module):
    def __init__(self):
        super(Network_1_type1, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(1, 2),
            nn.ReLU(),
            nn.Linear(2, 1)
        )

    def forward(self, x: torch.Tensor):
        return self.fc(x)
