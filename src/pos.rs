use std::fmt; //python:del

pub struct Pos(pub u64);

impl Pos {
    #[allow(dead_code)]
    pub fn new(x: u8, y: u8) -> Self {
        Pos(1 << (x + y * 8))
    }
    pub fn index(&self) -> (usize, usize) {
        let pos = 63 - self.0.leading_zeros() as usize;
        (pos % 8, pos / 8)
    }
}

const LABEL_X: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const LABEL_Y: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.index();
        unsafe {
            write!(
                f,
                "{}{}",
                LABEL_X.get_unchecked(x),
                LABEL_Y.get_unchecked(y)
            )
        }
    }
}
