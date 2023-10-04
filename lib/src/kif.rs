use crate::base::Board;
use crate::pos::PyPos;
use std::fmt;

pub struct Kif(Vec<PyPos>);

impl Kif {
    pub fn new(data: &str) -> Result<Self, &'static str> {
        let data = data.trim();
        let mut res = Vec::with_capacity(data.len() / 2);
        for i in (0..data.len()).step_by(2) {
            let p = PyPos::decode(&data[i..i + 2])?;
            res.push(p);
        }
        Ok(Kif(res))
    }

    pub fn parse(&self) -> Vec<Board> {
        let mut board = Board::new();
        let mut res = Vec::with_capacity(self.0.len() + 1);
        let mut rev = false;
        unsafe {
            for pos in self.0.iter() {
                let next = board.put(pos.to_pos());
                res.push(if rev { board.pass() } else { board });
                board = next;
                rev = !rev;
                if board.legal_moves() == 0 {
                    board = board.pass();
                    rev = !rev;
                }
            }
            res.push(board);
        }
        res
    }

    pub fn parse_with_check(&self) -> Result<Vec<Board>, String> {
        let mut board = Board::new();
        let mut res = Vec::with_capacity(self.0.len() + 1);
        let mut rev = false;
        unsafe {
            for (index, p) in self.0.iter().enumerate() {
                let pos = p.to_pos();
                if board.legal_moves() & pos.0 == 0 {
                    let msg = format!("illegal move: {} ({}th move)", pos, index + 1);
                    return Err(msg);
                }
                let next = board.put(pos);
                res.push(if rev { board.pass() } else { board });
                board = next;
                rev = !rev;
                if board.legal_moves() == 0 {
                    board = board.pass();
                    rev = !rev;
                }
            }
            res.push(board);
        }
        Ok(res)
    }

    pub fn pysave(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.0.len());
        for k in self.0.iter() {
            v.push(k.to_bin());
        }
        v
    }

    pub fn pyload(data: &[u8]) -> Self {
        let mut v = Vec::with_capacity(data.len());
        for k in data.iter() {
            v.push(PyPos::new(*k));
        }
        Kif(v)
    }
    pub fn rotate(&self) -> Self {
        let mut v = Vec::with_capacity(self.0.len());
        for k in self.0.iter() {
            v.push(k.rotate());
        }
        Kif(v)
    }

    pub fn miror(&self) -> Self {
        let mut v = Vec::with_capacity(self.0.len());
        for k in self.0.iter() {
            v.push(k.mirror());
        }
        Kif(v)
    }
}

impl fmt::Display for Kif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for k in self.0.iter() {
            write!(f, "{}", k)?;
        }
        Ok(())
    }
}
