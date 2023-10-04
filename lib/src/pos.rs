use std::fmt;

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
    #[allow(dead_code)]
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
    pub fn from_py(idx: usize) -> Self {
        Pos(1 << idx)
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

pub struct PyPos(u8);

pub const MIROR: [u8; 64] = [
    0, 8, 16, 24, 32, 40, 48, 56, 1, 9, 17, 25, 33, 41, 49, 57, 2, 10, 18, 26, 34, 42, 50, 58, 3,
    11, 19, 27, 35, 43, 51, 59, 4, 12, 20, 28, 36, 44, 52, 60, 5, 13, 21, 29, 37, 45, 53, 61, 6,
    14, 22, 30, 38, 46, 54, 62, 7, 15, 23, 31, 39, 47, 55, 63,
];

pub const ROTATE: [u8; 64] = [
    7, 15, 23, 31, 39, 47, 55, 63, 6, 14, 22, 30, 38, 46, 54, 62, 5, 13, 21, 29, 37, 45, 53, 61, 4,
    12, 20, 28, 36, 44, 52, 60, 3, 11, 19, 27, 35, 43, 51, 59, 2, 10, 18, 26, 34, 42, 50, 58, 1, 9,
    17, 25, 33, 41, 49, 57, 0, 8, 16, 24, 32, 40, 48, 56,
];

impl PyPos {
    pub fn new(data: u8) -> Self {
        PyPos(data)
    }
    pub fn decode(data: &str) -> Result<Self, &'static str> {
        let mut chars = data.chars();
        let x = chars.next().unwrap() as u8 - b'a';
        let y = chars.next().unwrap() as u8 - b'1';
        if x > 7 || y > 7 {
            return Err("invalid pos");
        }
        Ok(Self(x + y * 8))
    }
    pub fn to_bin(&self) -> u8 {
        self.0
    }
    pub fn to_pos(&self) -> Pos {
        Pos(1 << self.0)
    }
    pub fn x(&self) -> u8 {
        self.0 % 8
    }
    pub fn y(&self) -> u8 {
        self.0 / 8
    }
    pub fn mirror(&self) -> Self {
        Self(MIROR[self.0 as usize])
    }
    pub fn rotate(&self) -> Self {
        Self(ROTATE[self.0 as usize])
    }
}

impl fmt::Display for PyPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = (self.x(), self.y());
        unsafe {
            write!(
                f,
                "{}{}",
                LABEL_X.get_unchecked(x as usize),
                LABEL_Y.get_unchecked(y as usize)
            )
        }
    }
}
