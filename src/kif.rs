use crate::base::Board;
use crate::pos::Pos;

pub fn load_kif(data: &str) -> Result<Vec<Board>, &'static str> {
    let mut board = Board::new();
    let mut res = vec![];
    let data = data.trim();
    let len = data.len();
    let mut rev = false;
    unsafe {
        res.set_len(len / 2 + 1);
        for i in (0..data.len()).skip(2) {
            let next = board.put(Pos::from_str(&data[i..i + 2])?);
            *res.get_unchecked_mut(i / 2) = if rev { board.pass() } else { board };
            board = next;
            rev = !rev;
            if board.legal_moves() == 0 {
                board = board.pass();
                rev = !rev;
            }
        }
        *res.get_unchecked_mut(len / 2) = board;
    }
    Ok(res)
}
