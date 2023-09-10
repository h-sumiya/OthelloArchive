# pyothello

Python用オセロツールキット

## パフォーマンス
- put: 2,000,000回/秒
- legal_moves: 500,000回/秒
- edax: 160,000回/秒

## ビルド
`maturin build --release`
`pip install target/wheels/pyothello-xxx.whl`

## 使い方

```python
from pyothello import Board, Record, Pos, Boards

board = Board.default()
print(board)
print(board.data) # 0:空, 1:黒, 2:白
print(board[0])
print(board.legal_moves())
print(board.count)
print(int(board))
print(board.cns) #合法手の数
print(board.edax())

next_board = board.put(19)
print(next_board)

board2 = Board(bytes(board))
board2 == board # True

pos = Pos(10).mirror().rotate()

record = Record.load("f5d6...")
record_mirror = record.mirror()
record_rotate = record.rotate()
boards:Borads = record.parse() # parse_with_check()　合法手チェック
board = boards[10] # 10手目の盤面
```