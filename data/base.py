from .moves import MIROR_TABLE, ROTATE_TABLE
from dataclasses import dataclass
from enum import Enum
from copy import copy
from numba import jit


X_LABEL = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
Y_LABEL = ['1', '2', '3', '4', '5', '6', '7', '8']


@dataclass
class Pos:
    x: int
    y: int

    def load(val):
        if type(val) == int:
            return Pos(val % 8, val // 8)
        elif type(val) == str:
            x = X_LABEL.index(val[0])
            y = Y_LABEL.index(val[1])
            return Pos(x, y)

    def index(self):
        return self.y * 8 + self.x

    def __str__(self):
        return X_LABEL[self.x] + Y_LABEL[self.y]

    def __int__(self) -> int:
        return self.index()

    def __add__(self, other: 'Pos') -> 'Pos':
        return Pos(self.x + other.x, self.y + other.y)

    def valid(self) -> bool:
        return 0 <= self.x < 8 and 0 <= self.y < 8

    def __hash__(self) -> int:
        return hash(self.index())

    def rotate(self, n=1) -> 'Pos':
        res = self.index()
        for _ in range(n):
            res = ROTATE_TABLE[res]
        return Pos.load(res)

    def miror(self) -> 'Pos':
        new_index = MIROR_TABLE[self.index()]
        return Pos.load(new_index)


class Color(Enum):
    Null = 0
    Black = 1
    White = 2

    def __str__(self) -> str:
        if self == Color.Null:
            return " "
        if self == Color.Black:
            return "●"
        if self == Color.White:
            return "○"

    def other(self) -> 'Color':
        if self == Color.Black:
            return Color.White
        if self == Color.White:
            return Color.Black
        return Color.Null

    def __int__(self) -> int:
        if self == Color.Null:
            return 0
        elif self == Color.Black:
            return 1
        else:
            return -1

    def from_int(val: int) -> 'Color':
        if val == 0:
            return Color.Null
        elif val == 1:
            return Color.Black
        else:
            return Color.White

    @property
    def flag(self) -> int:
        return 0 if self == Color.Null else 1


@dataclass
class Board:
    data: list[Color]

    def load(data: bytes, color: Color) -> 'Board':
        data = int.from_bytes(data, "little", signed=False)
        board = [Color.Null] * 64
        for i in range(64):
            if data & (1 << i):
                board[i] = color
        return Board(board)

    def __getitem__(self, index) -> Color:
        if type(index) == int:
            return self.data[index]
        elif type(index) == Pos:
            return self.data[index.index()]
        elif type(index) == slice:
            return Board(self.data[index])
        elif type(index) == list:
            return [self[i] for i in index]
        else:
            raise Exception("Invalid index")

    def __str__(self) -> str:
        text = ""
        for y in range(8):
            for x in range(8):
                text += str(self.data[y*8+x])
            text += "\n"
        return text

    def __add__(self, other: 'Board') -> 'Board':
        l = [
            self[i] if self[i] != Color.Null else other[i]
            for i in range(64)
        ]
        return Board(l)
