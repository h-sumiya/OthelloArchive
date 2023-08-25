from data import Datas
from pathlib import Path
from indexes import *
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import torch
from Network import *

DATA_PATH = Path(__file__).parent / "datas"


def load_datas(id: int) -> Datas:
    path = DATA_PATH / f"{id}.bin"
    return Datas.load(path)


class OseroNetworks(nn.Module):
    def __init__(self):
        super(OseroNetworks, self).__init__()
        self.edge = Network_10_type1()
        self.cross = Network_8_type1()
        self.corner = Network_10_type2()
        self.cn = Network_1_type1()

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


if __name__ == "__main__":
    datas = load_datas(30)
    edges = []
    crosses = []
    corners = []
    cns = []
    outputs = []
    for data in datas:
        edges.append(data[EDGES2])
        crosses.append(data[CROSSES])
        corners.append(data[CORNERS])
        cns.append([data.cn])
        outputs.append(data.score)

    edges = torch.tensor(edges, dtype=torch.float32)
    crosses = torch.tensor(crosses, dtype=torch.float32)
    corners = torch.tensor(corners, dtype=torch.float32)
    cns = torch.tensor(cns, dtype=torch.float32)
    outputs = torch.tensor(outputs, dtype=torch.float32)

    dataset = TensorDataset(edges, crosses, corners, cns, outputs)
    dataloader = DataLoader(
        dataset,
        batch_size=256,
        shuffle=True,
        pin_memory=True,
        num_workers=2
    )

    model = OseroNetworks()
    criterion = nn.MSELoss()
    optimizer = optim.Adam(model.parameters(), lr=0.003)

    for epoch in range(1000):
        running_loss = 0.0
        for i, data in enumerate(dataloader):
            edge, cross, corner, cn, output = data
            optimizer.zero_grad()
            outputs = model(edge, cross, corner, cn)
            loss = criterion(outputs, output)
            loss.backward()
            optimizer.step()
            running_loss += loss.item()
        print(f"[{epoch + 1}] loss: {running_loss / len(dataloader)}")
