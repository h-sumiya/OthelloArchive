use crate::base::Board;
use crate::pos::Pos;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static mut KILLER: Lazy<HashMap<(u64, u64), Pos>> = Lazy::new(|| {
    let mut data = HashMap::new();
    let mut board = Board::default();
    for d in include_bytes!("book.bin").iter() {
        if d == &255 {
            board = Board::default();
            continue;
        }
        let p = Pos(1 << d);
        data.insert((board.me, board.opp), p);
        board = board.put(p);
    }
    data
});
