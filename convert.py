import torch
from tools import *
from Network import OseroNetworks
import struct
from pprint import pprint
from data import Datas
from path import DataPath
from indexes import *
from torch.utils.data import DataLoader


def convert(lin: torch.nn.Linear):
    data = b""
    for ws in lin.weight:
        for w in ws:
            f = float(w)
            data += struct.pack("f", f)
            print(f)
    for b in lin.bias:
        f = float(b)
        print(b, type(b))
        data += struct.pack("f", f)
    return data


def temp():
    model = OseroNetworks.load(30)
    path = DataPath.id(30)
    datas = Datas.load(path, 10)
    dataset = datas.to_dataset([
        EDGES2,
        CROSSES,
        CORNERS
    ])
    dataloader = DataLoader(
        dataset,
        batch_size=1,
        shuffle=False,
        pin_memory=True,
        num_workers=3
    )
    for data in datas:
        print(data.board.codingame())
        print(data.score)

    for i, data in enumerate(dataloader):
        print(i)
        print(datas[i].board.codingame())
        edge, cross, corner, cn, output = data
        socre = model(edge, cross, corner, cn)
        print(socre)


def dump(id):
    model = OseroNetworks.load(id)
    edge = model.edge.convert()
    corner = model.corner.convert()
    cross = model.cross.convert()
    cn = model.cn.convert(False)
    data = edge + corner + cross + cn
    data = struct.pack("f" * len(data), *data)
    return data


if __name__ == "__main__":
    data = b""
    for i in range(10, 43):
        data += dump(i)
    rsb = RustBuilder()
    rsb.add(data, Header(
        "SCORE_DATA",
        f"[f32; {len(data) // 4}]",
        pub=True
    ))
    rsb.build()
