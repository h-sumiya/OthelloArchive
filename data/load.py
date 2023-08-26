from .base import Board, Color
import struct
from pathlib import Path
from dataclasses import dataclass
from io import BytesIO
from tqdm import trange, tqdm
from torch import tensor
import torch
from torch.utils.data import TensorDataset


@dataclass
class Data:
    me: Board
    opp: Board
    score: float
    cn: int
    board: Board | None = None

    def load(stream: BytesIO) -> 'Data':
        return Data(
            Board.load(stream.read(8), Color.Black),
            Board.load(stream.read(8), Color.White),
            struct.unpack('d', stream.read(8)),
            int.from_bytes(stream.read(8), byteorder='little', signed=True)
        )

    def load_reverse(stream: BytesIO) -> 'Data':
        white = Board.load(stream.read(8), Color.White)
        black = Board.load(stream.read(8), Color.Black)
        return Data(
            black,
            white,
            - struct.unpack('d', stream.read(8)),
            - int.from_bytes(stream.read(8), byteorder='little', signed=True)
        )

    def __str__(self) -> str:
        data = self.me + self.opp
        return f"{data}\nscore: {self.score}\ncn: {self.cn}\n"

    def to_val(colors) -> any:
        if type(colors) == Color:
            return int(colors)
        elif type(colors) == list:
            return [Data.to_val(color) for color in colors]

    def read(self, indexes) -> any:
        if self.board is None:
            self.board = self.me + self.opp
        data = self.board[indexes]
        return Data.to_val(data)

    def __getitem__(self, index) -> any:
        return self.read(index)

    def to_flag(blacks, whites) -> any:
        if type(blacks[0]) == Color:
            res = [b.flag for b in blacks] + [w.flag for w in whites]
            return res
        else:
            return [Data.to_flag(b, w) for b, w in zip(blacks, whites)]

    def read_flag(self, indexes) -> any:
        me, opp = self.me[indexes] + self.opp[indexes]
        return Data.to_flag(me, opp)


APP_ROOT = Path(__file__).parent


@dataclass
class Datas:
    data: list[Data]

    def read(path) -> bytes:
        path = Path(path)
        return path.read_bytes()

    def load(path, size=0, reverse=False) -> 'Datas':
        data = Datas.read(path)
        if size == 0:
            size = len(data) // 32
        else:
            size = min(size, len(data) // 32)
        res = []
        stream = BytesIO(data)
        pbar = trange(size)
        pbar.set_description("Unpacking Data...")
        if reverse:
            for _ in pbar:
                res.append(Data.load_reverse(stream.read(32)))
        else:
            for _ in pbar:
                res.append(Data.load(stream))

        return Datas(res)

    def get(self, index):
        return self.data[index]

    def __getitem__(self, index) -> Data | list[Data]:
        if type(index) == int:
            return self.get(index)
        else:
            return Datas(self.data[index])

    def __len__(self) -> int:
        return len(self.data)

    def __str__(self) -> str:
        return f"<Datas: {len(self)} datas>"

    class Iterator:
        def __init__(self, datas):
            self.datas = datas
            self.index = 0

        def __next__(self) -> Data:
            if self.index >= len(self.datas):
                raise StopIteration()
            self.index += 1
            return self.datas[self.index - 1]

    def to_dataset(self, indexes, to_tensor=True, to_dataset=True) -> any:
        datas = []
        cns = []
        scores = []
        for i, index in enumerate(indexes):
            pbar = tqdm(self)
            pbar.set_description(f"Converting Data{i}...")
            index_data = []
            for data in pbar:
                index_data.append(data[index])
            datas.append(index_data)
        pbar = tqdm(self)
        pbar.set_description("Converting CN and Score...")
        for data in pbar:
            cns.append([data.cn])
            scores.append(data.score)
        if to_tensor:
            datas = [tensor(data, dtype=torch.float32) for data in datas]
            cns = tensor(cns, dtype=torch.float32)
            scores = tensor(scores, dtype=torch.float32)
            if to_dataset:
                return TensorDataset(*datas, cns, scores)
        return datas, cns, scores
