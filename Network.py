import torch.nn as nn
import torch
from path import *


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


class Network_8_type2(nn.Module):
    def __init__(self):
        super(Network_8_type2, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(8, 16),
            nn.ReLU(),
            nn.Linear(16, 8),
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


class Network_1_type2(nn.Module):
    def __init__(self):
        super(Network_1_type2, self).__init__()
        self.fc = nn.Linear(1, 1)

    def forward(self, x: torch.Tensor):
        return self.fc(x)


class OseroNetworks(nn.Module):
    def __init__(self):
        super(OseroNetworks, self).__init__()
        self.edge = Network_10_type1()
        self.cross = Network_8_type1()
        self.corner = Network_10_type1()
        self.cn = Network_1_type2()

    def forward(self, edge: torch.Tensor, cross: torch.Tensor, corner: torch.Tensor, cn: torch.Tensor):
        res = []
        edges = edge.unbind(dim=1)
        for edge in edges:
            res.append(self.edge(edge))
        crosses = cross.unbind(dim=1)
        for cross in crosses:
            res.append(self.cross(cross))
        corners = corner.unbind(dim=1)
        for corner in corners:
            res.append(self.corner(corner))
        res.append(self.cn(cn))
        return torch.sum(torch.stack(res), dim=0)

    def save(self, id: int):
        path = ModelPath.id(id)
        torch.save(self.state_dict(), path)

    def load(id: int) -> "OseroNetworks":
        path = ModelPath.id(id)
        model = OseroNetworks()
        model.load_state_dict(torch.load(path))
        model.eval()
        return model
