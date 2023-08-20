 use std::fmt;
 use std::io;
 
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

 
pub fn read_usize(buf: &mut String) -> usize {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
    buf.trim().parse().unwrap()
}

pub fn read_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
    buf.pop();
}

pub fn skip_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
}

pub fn skip_lines(buf: &mut String, n: usize) {
    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(buf).unwrap();
    }
}

pub fn read_id(buf: &mut String) -> char {
    io::stdin().read_line(buf).unwrap();
    let id = buf.trim().chars().next().unwrap();
    skip_line(buf);
    id
}

pub fn skip_after(buf: &mut String) {
    let num = read_usize(buf);
    skip_lines(buf, num);
}

 //参考:https://qiita.com/watergreat31/items/09f114799319c7ba19dc
pub const BOTTOM_MASK: u64 = 0xFFFFFFFFFFFFFF00;
pub const TOP_MASK: u64 = 0x00FFFFFFFFFFFFFF;
pub const LEFT_MASK: u64 = 0x7F7F7F7F7F7F7F7F;
pub const RIGHT_MASK: u64 = 0xFEFEFEFEFEFEFEFE;
pub const HORIZONTAL_MASK: u64 = RIGHT_MASK & LEFT_MASK;
pub const VERTICAL_MASK: u64 = TOP_MASK & BOTTOM_MASK;
pub const DIAGONAL_MASK: u64 = HORIZONTAL_MASK & VERTICAL_MASK;
pub const TOP_LEFT_MASK: u64 = TOP_MASK & LEFT_MASK;
pub const TOP_RIGHT_MASK: u64 = TOP_MASK & RIGHT_MASK;
pub const BOTTOM_LEFT_MASK: u64 = BOTTOM_MASK & LEFT_MASK;
pub const BOTTOM_RIGHT_MASK: u64 = BOTTOM_MASK & RIGHT_MASK;

const fn all_pos_mask() -> [u64; 64] {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = 1 << i;
        i += 1;
    }
    table
}
pub const POSES: [u64; 64] = all_pos_mask();

 
 use std::time::Instant;

const LiMIT_TIME: u128 = 140;
pub struct TimeManager {
    start: Instant,
    last: u128,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            last: 0,
        }
    }

    pub fn lap(&mut self) -> Result<(), ()> {
        let elapsed = self.start.elapsed().as_millis();
        let diff = elapsed - self.last;
        if elapsed + diff > LiMIT_TIME {
            return Err(());
        } else {
            self.last = elapsed;
            return Ok(());
        }
    }
}

 
const fn calc_fs(i: u8, j: u8) -> i8 {
    let mut socre = 0;
    if i & j != 0 {
        return 0;
    }
    if i | j == 255 {
        socre = i.count_ones() as i8 - j.count_ones() as i8;
    } else {
        socre += i.leading_ones() as i8;
        socre += i.trailing_ones() as i8;
        socre -= j.leading_ones() as i8;
        socre -= j.trailing_ones() as i8;
    }
    socre *= 2;
    if i & 1 != 0 {
        socre -= 1;
    }
    if i & 128 != 0 {
        socre -= 1;
    }
    if j & 1 != 0 {
        socre += 1;
    }
    if j & 128 != 0 {
        socre += 1;
    }
    socre
}

const fn calc_fses() -> [[i8; 256]; 256] {
    let mut table = [[0i8; 256]; 256];
    let mut i = 0u8;
    loop {
        let mut j = 0u8;
        loop {
            let socre = calc_fs(i, j);
            table[i as usize][j as usize] = socre;
            if j == 255 {
                break;
            }
            j += 1;
        }
        if i == 255 {
            break;
        }
        i += 1;
    }
    table
}

const fn calc_point() -> [[isize; 256]; 8] {
    let mut table = [[0; 256]; 8];
    let data = [
        [45, -11, 4, -1, -1, 4, -11, 45],
        [-11, -16, -1, -3, -3, -1, -16, -11],
        [4, -1, 2, -1, -1, 2, -1, 4],
        [-1, -3, -1, 0, 0, -1, -3, -1],
        [-1, -3, -1, 0, 0, -1, -3, -1],
        [4, -1, 2, -1, -1, 2, -1, 4],
        [-11, -16, -1, -3, -3, -1, -16, -11],
        [45, -11, 4, -1, -1, 4, -11, 45],
    ];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 256 {
            let mut point = 0;
            let mut k = 0;
            while k < 8 {
                if j & (1 << k) != 0 {
                    point += data[i][k];
                }
                k += 1;
            }
            table[i][j] = point;
            j += 1;
        }
        i += 1;
    }
    table
}

const POINT_TABLE: [[isize; 256]; 8] = calc_point();
pub const FS_TABLE: [[i8; 256]; 256] = calc_fses();

//[参考]https://qiita.com/ysuzuk81/items/9ee9d0a295471bb6d1ef
fn get_top_edge(board: u64) -> usize {
    ((board & 0xFF00000000000000) >> 56) as usize
}

fn get_bottom_edge(board: u64) -> usize {
    (board & 0x00000000000000FF) as usize
}

fn get_left_edge(board: u64) -> usize {
    ((board & 0x8080808080808080) * 0x0002040810204081 >> 56) as usize
}

fn get_right_edge(board: u64) -> usize {
    ((board & 0x0101010101010101) * 0x0102040810204080 >> 56) as usize
}

impl Board {
    pub fn count(&self) -> isize {
        self.me.count_ones() as isize + self.opp.count_ones() as isize
    }

    pub fn remain(&self) -> usize {
        64 - self.count() as usize
    }

    pub fn kn(&self) -> isize {
        self.me.count_ones() as isize - self.opp.count_ones() as isize
    }

    pub fn cn(&self) -> isize {
        let me = self.legal_moves().count_ones() as isize;
        let opp = self.opp_legal_moves().count_ones() as isize;
        me - opp
    }

    pub fn bp(&self) -> isize {
        let mut points = 0;
        unsafe {
            for (i, table) in POINT_TABLE.iter().rev().enumerate() {
                let me_index = ((self.me >> (i * 8)) & 0x00000000000000FF) as usize;
                points += *table.get_unchecked(me_index);
                let opp_index = ((self.opp >> (i * 8)) & 0x00000000000000FF) as usize;
                points -= *table.get_unchecked(opp_index);
            }
        }
        points
    }

    pub fn fs(&self) -> isize {
        let mut score = 0i8;
        unsafe {
            score += *FS_TABLE
                .get_unchecked(get_top_edge(self.me))
                .get_unchecked(get_top_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_bottom_edge(self.me))
                .get_unchecked(get_bottom_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_left_edge(self.me))
                .get_unchecked(get_left_edge(self.opp));
            score += *FS_TABLE
                .get_unchecked(get_right_edge(self.me))
                .get_unchecked(get_right_edge(self.opp));
        }
        score as isize
    }

    pub fn score(&self) -> isize {
        self.cn() * 20 + self.bp() * 2 + self.fs() * 75
    }
}

 
const MIN: isize = -10000;
const MAX: isize = 10000;
impl Board {
    pub fn ai(&self) -> Pos {
        let remain = self.remain();
        let pos;
        if remain > 50 {
            pos = self.arufa_beta(7);
        } else if remain > 13 {
            pos = self.arufa_beta(7);
        } else {
            pos = self.full_search(remain);
        }
        pos
    }

    pub fn arufa_beta(&self, depth: usize) -> Pos {
        let moves = self.legal_moves();
        let mut scores = vec![];
        for pos in POSES {
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.arufa_beta_node(2, MIN, MAX);
                scores.push((score, pos));
            }
        }
        scores.sort_by_key(|x| -x.0);
        let mut max = MIN;
        let mut max_pos = 0;
        for (_, pos) in scores {
            let board = self.put(Pos(pos));
            let score = -board.arufa_beta_node(depth - 1, MIN, -max);
            if score > max {
                max = score;
                max_pos = pos;
            }
        }
        Pos(max_pos)
    }

    fn arufa_beta_node(&self, depth: usize, mut a: isize, b: isize) -> isize {
        if depth == 0 {
            return self.score();
        }
        let mut moves = self.legal_moves();
        if moves == 0 {
            moves = self.opp_legal_moves();
            if moves == 0 {
                return self.score();
            }
            return -self.pass().arufa_beta_node(depth - 1, -b, -a);
        }

        for pos in POSES {
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.arufa_beta_node(depth - 1, -b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
            }
        }
        a
    }

    pub fn full_search(&self, remain: usize) -> Pos {
        let start = std::time::Instant::now();
        let moves = self.legal_moves();
        if remain <= 4 {
            let mut max = MIN;
            let mut max_pos = 0;
            for pos in POSES {
                if moves & pos != 0 {
                    let board = self.put(Pos(pos));
                    let score = -board.full_search_node(remain, MIN, -max);
                    if score > max {
                        max = score;
                        max_pos = pos;
                    }
                }
            }
            Pos(max_pos)
        } else {
            let mut scores = vec![];
            for pos in POSES {
                if moves & pos != 0 {
                    let board = self.put(Pos(pos));
                    let score = -board.full_search_node(2, MIN, MAX);
                    scores.push((score, pos));
                }
            }
            scores.sort_by_key(|x| -x.0);
            let mut max = MIN;
            let mut max_pos = 0;
            for (_, pos) in scores {
                let board = self.put(Pos(pos));
                let score = -board.full_search_node(remain, MIN, -max);
                if score > max {
                    max = score;
                    max_pos = pos;
                }
                if start.elapsed().as_millis() > 120 {
                    break;
                }
            }
            Pos(max_pos)
        }
    }

    fn full_search_node(&self, depth: usize, a: isize, b: isize) -> isize {
        if depth == 0 {
            return self.kn();
        }
        let mut moves = self.legal_moves();
        if moves == 0 {
            moves = self.opp_legal_moves();
            if moves == 0 {
                return self.kn();
            }
            return -self.pass().arufa_beta_node(depth - 1, -b, -a);
        }

        let mut a = a;
        for pos in POSES {
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.arufa_beta_node(depth - 1, -b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
            }
        }
        a
    }
}



fn main() {
    let mut buf = String::new();
    let id = read_id(&mut buf);
    let mut board = Board::new();
    loop {
        for _ in 0..8 {
            read_line(&mut buf);
            board.push(&buf, id);
        }
        skip_after(&mut buf);
        let pos = board.ai();
        println!("{}", pos);
    }
}
