from dataclasses import dataclass
from .pyothello import *
from .table import *
from typing import Union


X_LABELS = "abcdefgh"
Y_LABELS = "12345678"


@dataclass
class Pos:
    index: int

    def load(x: int, y: int) -> "Pos":
        """
        指定されたx座標とy座標からPosオブジェクトを生成します。

        Args:
            x (int): x座標
            y (int): y座標

        Returns:
            Pos: 生成されたPosオブジェクト
        """
        return Pos(x * 8 + y)

    def from_str(s: str) -> "Pos":
        """
        指定された文字列からPosオブジェクトを生成します。

        Args:
            s (str): 文字列（例: "a1"）

        Returns:
            Pos: 生成されたPosオブジェクト
        """
        return Pos(X_LABELS.index(s[0]) * 8 + Y_LABELS.index(s[1]))

    @property
    def x(self) -> int:
        """
        Posオブジェクトのx座標を取得します。

        Returns:
            int: x座標
        """
        return self.index // 8

    @property
    def y(self) -> int:
        """
        Posオブジェクトのy座標を取得します。

        Returns:
            int: y座標
        """
        return self.index % 8

    def __str__(self) -> str:
        """
        Posオブジェクトを文字列に変換します。

        Returns:
            str: 文字列（例: "a1"）
        """
        return f"{X_LABELS[self.x]}{Y_LABELS[self.y]}"

    def __eq__(self, other: "Pos") -> bool:
        """
        2つのPosオブジェクトが等しいかどうかを判定します。

        Args:
            other (Pos): 比較対象のPosオブジェクト

        Returns:
            bool: 2つのPosオブジェクトが等しい場合はTrue、そうでない場合はFalse
        """
        return self.index == other.index

    def __ne__(self, other: "Pos") -> bool:
        """
        2つのPosオブジェクトが等しくないかどうかを判定します。

        Args:
            other (Pos): 比較対象のPosオブジェクト

        Returns:
            bool: 2つのPosオブジェクトが等しくない場合はTrue、そうでない場合はFalse
        """
        return self.index != other.index

    def __add__(self, other: "Pos") -> "Pos":
        """
        2つのPosオブジェクトを加算します。

        Args:
            other (Pos): 加算するPosオブジェクト

        Returns:
            Pos: 加算結果のPosオブジェクト
        """
        return Pos.load(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Pos") -> "Pos":
        """
        2つのPosオブジェクトを減算します。

        Args:
            other (Pos): 減算するPosオブジェクト

        Returns:
            Pos: 減算結果のPosオブジェクト
        """
        return Pos.load(self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        """
        Posオブジェクトのハッシュ値を取得します。

        Returns:
            int: ハッシュ値
        """
        return hash(self.index)

    def __int__(self) -> int:
        """
        Posオブジェクトを整数に変換します。

        Returns:
            int: 整数値
        """
        return self.index

    def miror(self) -> "Pos":
        """
        Posオブジェクトを反転させたPosオブジェクトを取得します。

        Returns:
            Pos: 反転させたPosオブジェクト
        """
        return Pos(MIRROR[self.index])

    def rotate(self) -> "Pos":
        """
        Posオブジェクトを回転させたPosオブジェクトを取得します。

        Returns:
            Pos: 回転させたPosオブジェクト
        """
        return Pos(ROTATE[self.index])


@dataclass
class Board:
    _data: bytes
    _pydata: list[int] | None = None
    # 空0, 黒1, 白2

    def default() -> "Board":
        """
        初期盤面を返す

        Returns:
            Board: 初期盤面
        """
        return Board(_make())

    def load(data: list[int]) -> "Board":
        """
        盤面データを読み込んでBoardオブジェクトを生成する

        Args:
            data (list[int]): 盤面データ

        Returns:
            Board: 盤面オブジェクト
        """
        return Board(_mk_data(data))

    @property
    def data(self) -> list[int]:
        """
        盤面データを取得する

        Returns:
            list[int]: 盤面データ
        """
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
        """
        合法手のリストを取得する

        Returns:
            list[list[Pos]]: 合法手のリスト
        """
        return [[Pos(i) for i in j] for j in _legal_moves(self._data)]

    def me_legal_moves(self) -> list[Pos]:
        """
        自分の合法手のリストを取得する

        Returns:
            list[Pos]: 自分の合法手のリスト
        """
        return [Pos(i) for i in _me_legal_moves(self._data)]

    def opp_legal_moves(self) -> list[Pos]:
        """
        相手の合法手のリストを取得する

        Returns:
            list[Pos]: 相手の合法手のリスト
        """
        return [Pos(i) for i in _opp_legal_moves(self._data)]

    def put(self, pos: Pos, color=1) -> "Board":
        """
        盤面に石を置いた後の新しい盤面を返す

        Args:
            pos (Pos): 石を置く位置
            color (int, optional): 石の色. Defaults to 1.

        Returns:
            Board: 新しい盤面
        """
        if color == 1:
            return Board(_put(self._data, int(pos)))
        else:
            return Board(_put_opp(self._data, int(pos)))

    @property
    def count(self) -> int:
        """
        石の数を取得する

        Returns:
            int: 石の数
        """
        return _count(self._data)

    @property
    def counts(self) -> tuple[int, int]:
        """
        黒石と白石の数を取得する

        Returns:
            tuple[int, int]: 黒石の数, 白石の数
        """
        return _counts(self._data)

    @property
    def cns(self) -> tuple[int, int]:
        """
        候補数を取得する

        Returns:
            tuple[int, int]: 候補数
        """
        return _cns(self._data)

    def __int__(self) -> int:
        """
        盤面の評価値を取得する

        Returns:
            int: 盤面の評価値
        """
        return self.count - 4

    def __bytes__(self) -> bytes:
        """
        盤面データをバイト列で取得する

        Returns:
            bytes: 盤面データのバイト列
        """
        return self._data

    def __str__(self) -> str:
        """
        盤面の文字列表現を取得する

        Returns:
            str: 盤面の文字列表現
        """
        self._init_data()
        res = ""
        for y in range(8):
            for x in range(8):
                res += str(self.data[x + y * 8])
            res += "\n"
        return res

    def edax(self) -> float:
        """
        盤面のEDAX評価値を取得する

        Returns:
            float: 盤面のEDAX評価値
        """
        return _edax(self._data)

    def __add__(self, other: "Board") -> "Boards":
        """
        2つの盤面を結合した新しい盤面を返す

        Args:
            other (Board): 結合する盤面

        Returns:
            Boards: 新しい盤面
        """
        return Boards(self._data + bytes(other))

    def pack(self, index: bytes) -> list[int]:
        """
        AI学習用のデータを取得する

        Args:
            index (bytes): 盤面のインデックス

        Returns:
            list[int]: AI学習用のデータ
        """
        return _pyindex(self._data, index)


@dataclass
class Record:
    _data: bytes

    def load(data: str) -> "Record":
        """
        データを読み込んでRecordオブジェクトを作成します。

        Args:
            data (str): オセロの記録データ

        Returns:
            Record: 読み込まれたオセロの記録

        """
        return Record(_mk_kif(data))

    def __str__(self) -> str:
        """
        オセロの記録を文字列として返します。

        Returns:
            str: オセロの記録の文字列表現

        """
        return _read_kif(self._data)

    def rotate(self) -> "Record":
        """
        オセロの記録を左に90度回転させます。

        Returns:
            Record: 回転後のオセロの記録

        """
        return Record(_rotate_kif(self._data))

    def miror(self) -> "Record":
        """
        オセロの記録をa1-h8の対角線で反転させます。

        Returns:
            Record: 反転後のオセロの記録

        """
        return Record(_miror_kif(self._data))

    def __bytes__(self) -> bytes:
        """
        オセロの記録をバイト列として返します。

        Returns:
            bytes: オセロの記録のバイト列

        """
        return self._data

    def __len__(self) -> int:
        """
        記録されているオセロの手数を返します。

        Returns:
            int: オセロの手数

        """
        return len(self._data) + 1

    def parse(self) -> "Boards":
        """
        オセロの記録から0~60手目までの盤面を取得します。

        Returns:
            Boards: 盤面のリスト

        """
        return Boards(_parse_kif(self._data))

    def parse_with_check(self) -> "Boards":
        """
        オセロの記録から0~60手目までの盤面を取得します（合法手チェック付き）。

        Returns:
            Boards: 盤面のリスト

        """
        return Boards(_parse_kif_with_check(self._data))


@dataclass
class Boards:
    _data: bytes

    def __bytes__(self) -> bytes:
        """
        盤面データをバイト列として返します。

        Returns:
            bytes: 盤面データのバイト列

        """
        return self._data

    def __len__(self) -> int:
        """
        盤面の数を返します。

        Returns:
            int: 盤面の数

        """
        return len(self._data) // 16

    def __getitem__(self, index: int | slice) -> Union[Board, "Boards"]:
        """
        指定されたインデックスまたはスライスに対応する盤面を取得します。

        Args:
            index (int | slice): 取得する盤面のインデックスまたはスライス

        Returns:
            Union[Board, Boards]: 盤面または盤面のリスト

        """
        if isinstance(index, slice):
            start = index.start or 0
            stop = index.stop or len(self)
            step = index.step or 1
            if stop < 0:
                stop = len(self) + stop
            if start < 0:
                start = len(self) + start
            return Boards(self._data[start*16:stop*16:step*16])
        else:
            start = index * 16
            if index == -1:
                return Board(self._data[start:])
            return Board(self._data[start:start+16])

    class Iterator:
        def __init__(self, boards: "Boards"):
            self.boards = boards
            self.index = 0
            self.length = len(boards)

        def __iter__(self):
            return self

        def __next__(self):
            if self.index >= self.length:
                raise StopIteration()
            res = self.boards[self.index]
            self.index += 1
            return res

    def __iter__(self):
        """
        盤面をイテレーションするためのイテレータを返します。

        Returns:
            Boards.Iterator: 盤面のイテレータ

        """
        return Boards.Iterator(self)

    def __str__(self) -> str:
        """
        盤面の文字列表現を返します。

        Returns:
            str: 盤面の文字列表現

        """
        return f"<Boards: {len(self)}>"

    def __add__(self, other: Union[Board, "Boards"]) -> "Boards":
        """
        盤面を追加した新しいBoardsオブジェクトを返します。

        Args:
            other (Union[Board, Boards]): 追加する盤面または盤面のリスト

        Returns:
            Boards: 盤面を追加した新しいBoardsオブジェクト

        """
        return Boards(self._data + bytes(other))
