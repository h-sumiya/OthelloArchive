from dataclasses import dataclass
from .pyothello import *

X_LABELS = "abcdefgh"
Y_LABELS = "12345678"


@dataclass
class Pos:
    index: int

    def load(x: int, y: int) -> "Pos":
        return Pos(x * 8 + y)

    @property
    def x(self) -> int:
        return self.index // 8

    @property
    def y(self) -> int:
        return self.index % 8

    def __str__(self) -> str:
        return f"{X_LABELS[self.x]}{Y_LABELS[self.y]}"

    def __eq__(self, other: "Pos") -> bool:
        return self.index == other.index

    def __ne__(self, other: "Pos") -> bool:
        return self.index != other.index

    def __add__(self, other: "Pos") -> "Pos":
        return Pos.load(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Pos") -> "Pos":
        return Pos.load(self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        return hash(self.index)

    def __int__(self) -> int:
        return self.index


@dataclass
class Board:
    _data: bytes
    _pydata: list[int] | None = None

    def default() -> "Board":
        return Board(_make())

    def load(data: list[int]) -> "Board":
        return Board(_mk_data(data))

    @property
    def data(self) -> list[int]:
        if self._pydata is None:
            self._pydata = _decode(self._data)
        return self._pydata

    def _init_data(self) -> None:
        if self._pydata is None:
            self._pydata = _decode(self._data)

    def __getitem__(self, pos: Pos) -> int:
        if self._pydata is None:
            return _get(self._data, int(pos))
        else:
            return self._pydata[int(pos)]

    def legal_moves(self) -> list[list[Pos]]:
        return [[Pos(i) for i in j] for j in _legal_moves(self._data)]

    def me_legal_moves(self) -> list[Pos]:
        return [Pos(i) for i in _me_legal_moves(self._data)]

    def opp_legal_moves(self) -> list[Pos]:
        return [Pos(i) for i in _opp_legal_moves(self._data)]

    def put(self, pos: Pos, color=1) -> "Board":
        if color == 1:
            return Board(_put(self._data, int(pos)))
        else:
            return Board(_put_opp(self._data, int(pos)))

    class BoardIterator:
        def __init__(self, data: bytes):
            self.data = data
            self.pos = 0

        def __iter__(self) -> "Board.BoardIterator":
            return self

        def __next__(self) -> "Board":
            if self.pos >= len(self.data):
                raise StopIteration
            res = Board(self.data[self.pos:self.pos+16])
            self.pos += 16
            return res

    def load_stream(stream: str) -> "BoardIterator":
        res = _load_kif(stream)
        return Board.BoardIterator(res)

    @property
    def count(self) -> int:
        return _count(self._data)

    @property
    def counts(self) -> tuple[int, int]:
        return _counts(self._data)

    @property
    def cns(self) -> tuple[int, int]:
        return _cns(self._data)

    def __int__(self) -> int:
        return self.count - 4

    def save(self) -> bytes:
        return self._data

    def __str__(self) -> str:
        self._init_data()
        res = ""
        for y in range(8):
            for x in range(8):
                res += str(self.data[x + y * 8])
            res += "\n"
        return res

    def edax(self) -> float:
        return _edax(self._data)
