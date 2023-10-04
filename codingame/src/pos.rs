use std::fmt; //python:del

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub u64);
pub const LABEL_X: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const LABEL_Y: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl Pos {
    #[allow(dead_code)]
    pub fn new(x: u8, y: u8) -> Self {
        Pos(1 << (x + y * 8))
    }
    pub fn index(&self) -> (usize, usize) {
        let pos = 63 - self.0.leading_zeros() as usize;
        (pos % 8, pos / 8)
    }
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut chars = s.chars();
        let x = chars.next().unwrap() as u8 - b'a';
        let y = chars.next().unwrap() as u8 - b'1';
        if x > 7 || y > 7 {
            return Err("invalid pos");
        }
        Ok(Pos::new(x, y))
    }
    pub fn idx(&self) -> usize {
        63 - self.0.leading_zeros() as usize
    }
}

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
