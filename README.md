# pyothello

Python用オセロツールキット

## パフォーマンス
- put: 2,000,000回/秒
- legal_moves: 500,000回/秒
- edax: 160,000回/秒

## 使い方

```python
from pyothello import Board

board = Board.default()
print(board)
print(board.data) # 0:空, 1:黒, 2:白
print(board[0])
print(board.legal_moves())
print(board.count)
print(int(board))
print(board.edax())

next_board = board.put(19)
print(next_board)
```