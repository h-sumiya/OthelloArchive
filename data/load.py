from .base import Board, Color, Pos
import struct
from pathlib import Path
from dataclasses import dataclass


@dataclass
class Data:
    me: Board
    opp: Board
    score: float
    cn: int

    def load(data: bytes) -> 'Data':
        return Data(
            Board.load(data[0:8], Color.Black),
            Board.load(data[8:16], Color.White),
            struct.unpack('d', data[16:24]),
            int.from_bytes(data[24:32], byteorder='little', signed=True)
        )

    def load_reverse(data: bytes) -> 'Data':
        return Data(
            Board.load(data[8:16], Color.Black),
            Board.load(data[0:8], Color.White),
            - struct.unpack('d', data[16:24]),
            - int.from_bytes(data[24:32], byteorder='little', signed=True)
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
        data = (self.me + self.opp)[indexes]
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
        if reverse:
            for i in range(size):
                res.append(Data.load_reverse(data[i*32:(i+1)*32]))
        else:
            for i in range(size):
                res.append(Data.load(data[i*32:(i+1)*32]))

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
