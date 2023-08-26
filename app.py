from data import Datas
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader
import torch
from indexes import *
from Network import *
from path import *

CUDA = torch.device("cuda")


def load_datas(id: int) -> Datas:
    path = DataPath.id(id)
    return Datas.load(path)


def learn(id: int):
    datas = load_datas(id)
    dataset = datas.to_dataset([
        EDGES2,
        CROSSES,
        CORNERS
    ])

    dataloader = DataLoader(
        dataset,
        batch_size=256,
        shuffle=True,
        pin_memory=True,
        num_workers=3
    )

    model = OseroNetworks()
    model.to(CUDA)
    criterion = nn.MSELoss()
    optimizer = optim.Adam(model.parameters(), lr=0.003)

    for epoch in range(100):
        running_loss = 0.0
        for i, data in enumerate(dataloader):
            edge, cross, corner, cn, output = data
            edge = edge.to(CUDA)
            cross = cross.to(CUDA)
            corner = corner.to(CUDA)
            cn = cn.to(CUDA)
            output = output.to(CUDA)
            optimizer.zero_grad()
            outputs = model(edge, cross, corner, cn)
            loss = criterion(outputs, output)
            loss.backward()
            optimizer.step()
            running_loss += loss.item()
        print(f"[{epoch + 1}] loss: {running_loss / len(dataloader)}")

    model.save(id)


if __name__ == "__main__":
    learn(30)
