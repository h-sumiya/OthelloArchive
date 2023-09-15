use crate::mask::*;
use crate::pos::Pos;

pub struct Board {
    pub me: u64,
    pub opp: u64,
    pub count: usize,
}

const LAST_BIT: u64 = 1 << 63;
impl Board {
    pub fn default() -> Self {
        Self {
            me: 0x0000000810000000,
            opp: 0x0000001008000000,
            count: 0,
        }
    }
    pub fn new(data: Vec<usize>, id: usize) -> Self {
        let mut me = 0;
        let mut opp = 0;
        let mut count = 0;
        for d in data.iter() {
            me >>= 1;
            opp >>= 1;
            if *d == 0 {
                continue;
            } else if *d == id {
                me |= LAST_BIT;
                count += 1;
            } else {
                opp |= LAST_BIT;
                count += 1;
            }
        }
        count -= 4;
        Self { me, opp, count }
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
        let mut moves = buf << 1;

        buf = mask & (self.me >> 1);
        for _ in 0..5 {
            buf |= mask & (buf >> 1);
        }
        moves |= buf >> 1;

        mask = self.opp & VERTICAL_MASK;

        buf = mask & (self.me << 8);
        for _ in 0..5 {
            buf |= mask & (buf << 8);
        }
        moves |= buf << 8;

        buf = mask & (self.me >> 8);
        for _ in 0..5 {
            buf |= mask & (buf >> 8);
        }
        moves |= buf >> 8;

        mask = self.opp & DIAGONAL_MASK;

        buf = mask & (self.me << 7);
        for _ in 0..5 {
            buf |= mask & (buf << 7);
        }
        moves |= buf << 7;

        buf = mask & (self.me >> 7);
        for _ in 0..5 {
            buf |= mask & (buf >> 7);
        }
        moves |= buf >> 7;

        buf = mask & (self.me << 9);
        for _ in 0..5 {
            buf |= mask & (buf << 9);
        }
        moves |= buf << 9;

        buf = mask & (self.me >> 9);
        for _ in 0..5 {
            buf |= mask & (buf >> 9);
        }
        moves |= buf >> 9;

        moves & blank
    }

    pub fn opp_legal_moves(&self) -> u64 {
        let blank = self.blank();
        let mut mask = self.me & HORIZONTAL_MASK;

        let mut buf = mask & (self.opp << 1);
        for _ in 0..5 {
            buf |= mask & (buf << 1);
        }
        let mut moves = buf << 1;

        buf = mask & (self.opp >> 1);
        for _ in 0..5 {
            buf |= mask & (buf >> 1);
        }
        moves |= buf >> 1;

        mask = self.me & VERTICAL_MASK;

        buf = mask & (self.opp << 8);
        for _ in 0..5 {
            buf |= mask & (buf << 8);
        }
        moves |= buf << 8;

        buf = mask & (self.opp >> 8);
        for _ in 0..5 {
            buf |= mask & (buf >> 8);
        }
        moves |= buf >> 8;

        mask = self.me & DIAGONAL_MASK;

        buf = mask & (self.opp << 7);
        for _ in 0..5 {
            buf |= mask & (buf << 7);
        }
        moves |= buf << 7;

        buf = mask & (self.opp >> 7);
        for _ in 0..5 {
            buf |= mask & (buf >> 7);
        }
        moves |= buf >> 7;

        buf = mask & (self.opp << 9);
        for _ in 0..5 {
            buf |= mask & (buf << 9);
        }
        moves |= buf << 9;

        buf = mask & (self.opp >> 9);
        for _ in 0..5 {
            buf |= mask & (buf >> 9);
        }
        moves |= buf >> 9;

        moves & blank
    }

    pub fn put(&self, pos: Pos) -> Self {
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

        Self {
            me: opp,
            opp: me,
            count: self.count + 1,
        }
    }

    pub fn pass(&self) -> Self {
        Self {
            me: self.opp,
            opp: self.me,
            count: self.count,
        }
    }

    pub fn js_data(&self, me: usize, opp: usize) -> Vec<usize> {
        let mut res = vec![];
        for i in 0..64 {
            let pos = 1 << i;
            if pos & self.me != 0 {
                res.push(me);
            } else if pos & self.opp != 0 {
                res.push(opp);
            } else {
                res.push(0);
            }
        }
        res
    }
}
