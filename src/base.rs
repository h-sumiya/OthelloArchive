use crate::mask::*; //python:del
use crate::pos::Pos; //python:del

pub struct Board {
    pub me: u64,
    pub opp: u64,
}

const LAST_BIT: u64 = POSES[63];
impl Board {
    pub fn new() -> Self {
        Board { me: 0, opp: 0 }
    }

    pub fn push(&mut self, text: &str, id: char) {
        for c in text.chars() {
            self.me >>= 1;
            self.opp >>= 1;
            if c == '.' {
                continue;
            } else if id == c {
                self.me |= LAST_BIT;
            } else {
                self.opp |= LAST_BIT;
            }
        }
    }

    pub fn blank(&self) -> u64 {
        !(self.me | self.opp)
    }

    pub fn legal_moves(&self) -> u64 {
        let blank = self.blank();

        let mut mask = self.opp & HORIZONTAL_MASK;

        let mut buf = mask & (self.me << 1);
        for _ in 0..5 {
            buf |= mask & (buf << 1);
        }
        let mut moves = blank & (buf << 1);

        buf = mask & (self.me >> 1);
        for _ in 0..5 {
            buf |= mask & (buf >> 1);
        }
        moves |= blank & (buf >> 1);

        mask = self.opp & VERTICAL_MASK;

        buf = mask & (self.me << 8);
        for _ in 0..5 {
            buf |= mask & (buf << 8);
        }
        moves |= blank & (buf << 8);

        buf = mask & (self.me >> 8);
        for _ in 0..5 {
            buf |= mask & (buf >> 8);
        }
        moves |= blank & (buf >> 8);

        mask = self.opp & DIAGONAL_MASK;

        buf = mask & (self.me << 7);
        for _ in 0..5 {
            buf |= mask & (buf << 7);
        }
        moves |= blank & (buf << 7);

        buf = mask & (self.me >> 7);
        for _ in 0..5 {
            buf |= mask & (buf >> 7);
        }
        moves |= blank & (buf >> 7);

        buf = mask & (self.me << 9);
        for _ in 0..5 {
            buf |= mask & (buf << 9);
        }
        moves |= blank & (buf << 9);

        buf = mask & (self.me >> 9);
        for _ in 0..5 {
            buf |= mask & (buf >> 9);
        }
        moves |= blank & (buf >> 9);

        moves
    }

    pub fn opp_legal_moves(&self) -> u64 {
        let blank = self.blank();
        let mut mask = self.me & HORIZONTAL_MASK;

        let mut buf = mask & (self.opp << 1);
        for _ in 0..5 {
            buf |= mask & (buf << 1);
        }
        let mut moves = blank & (buf << 1);

        buf = mask & (self.opp >> 1);
        for _ in 0..5 {
            buf |= mask & (buf >> 1);
        }
        moves |= blank & (buf >> 1);

        mask = self.me & VERTICAL_MASK;

        buf = mask & (self.opp << 8);
        for _ in 0..5 {
            buf |= mask & (buf << 8);
        }
        moves |= blank & (buf << 8);

        buf = mask & (self.opp >> 8);
        for _ in 0..5 {
            buf |= mask & (buf >> 8);
        }
        moves |= blank & (buf >> 8);

        mask = self.me & DIAGONAL_MASK;

        buf = mask & (self.opp << 7);
        for _ in 0..5 {
            buf |= mask & (buf << 7);
        }
        moves |= blank & (buf << 7);

        buf = mask & (self.opp >> 7);
        for _ in 0..5 {
            buf |= mask & (buf >> 7);
        }
        moves |= blank & (buf >> 7);

        buf = mask & (self.opp << 9);
        for _ in 0..5 {
            buf |= mask & (buf << 9);
        }
        moves |= blank & (buf << 9);

        buf = mask & (self.opp >> 9);
        for _ in 0..5 {
            buf |= mask & (buf >> 9);
        }
        moves |= blank & (buf >> 9);

        moves
    }

    pub fn put(&self, pos: Pos) -> Self {
        //アルゴリズムを書き換えること(列と行のみTABLEを用意)
        let mut me = self.me;
        let mut opp = self.opp;
        let mut rev = 0;

        let mut move_pos = (pos.0 >> 1) & LEFT_MASK;
        let mut buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos >> 1) & LEFT_MASK;
        }
        if (move_pos & me) != 0 {
            rev = buf;
        }

        move_pos = (pos.0 << 1) & RIGHT_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos << 1) & RIGHT_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 << 8) & BOTTOM_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos << 8) & BOTTOM_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 >> 8) & TOP_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos >> 8) & TOP_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 << 7) & BOTTOM_LEFT_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos << 7) & BOTTOM_LEFT_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 >> 9) & TOP_LEFT_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos >> 9) & TOP_LEFT_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 << 9) & BOTTOM_RIGHT_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos << 9) & BOTTOM_RIGHT_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        move_pos = (pos.0 >> 7) & TOP_RIGHT_MASK;
        buf = 0;
        while move_pos != 0 && (move_pos & opp) != 0 {
            buf |= move_pos;
            move_pos = (move_pos >> 7) & TOP_RIGHT_MASK;
        }
        if (move_pos & me) != 0 {
            rev |= buf;
        }

        me ^= pos.0 | rev;
        opp ^= rev;

        Self { me: opp, opp: me }
    }

    pub fn pass(&self) -> Self {
        Self {
            me: self.opp,
            opp: self.me,
        }
    }
}