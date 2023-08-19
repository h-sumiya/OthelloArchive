//mod mask;
pub const HORIZONTAL: u64 = 0x7E7E7E7E7E7E7E7E;
pub const VERTICAL: u64 = 0x00FFFFFFFFFFFF00;
pub const DIAGONAL: u64 = 0x007E7E7E7E7E7E00;

macro_rules! check_direction {
    ($mask:expr, $player:expr,  >>  $offset:expr , $res:expr ,$blank:expr) => {
        let mut buf = $mask & ($player >> $offset);
        for _ in 0..5 {
            buf |= $mask & (buf >> $offset);
        }
        $res |= (buf >> $offset) & $blank;
    };
    ($mask:expr, $player:expr, << $offset:expr , $res:expr,$blank:expr) => {
        let mut buf = $mask & ($player << $offset);
        for _ in 0..5 {
            buf |= $mask & (buf << $offset);
        }
        $res |= (buf << $offset) & $blank;
    };
}

macro_rules! check_reverse {
    ($mask:expr, $pos:expr, $player:expr, >>  $offset:expr , $reverse:expr) => {
        let mut pos = $pos >> $offset;
        let mut reverse = 0;
        while pos & $mask != 0 {
            reverse |= pos;
            pos >>= $offset;
        }
        if pos & $player != 0 {
            $reverse |= reverse;
        }
    };
    ($mask:expr, $pos:expr, $player:expr, <<  $offset:expr , $reverse:expr) => {
        let mut pos = $pos << $offset;
        let mut reverse = 0;
        while pos & $mask != 0 {
            reverse |= pos;
            pos <<= $offset;
        }
        if pos & $player != 0 {
            $reverse |= reverse;
        }
    };
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

use std::io;

fn go_to_dustbox(buf: &mut String) {
    io::stdin().read_line(buf).unwrap();
    buf.clear();
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let id = parse_input!(buf, usize);
    let player = if id == 0 {
        Player::Black
    } else {
        Player::White
    };
    go_to_dustbox(&mut buf);

    loop {
        let board = Board::from_stdin(player);
        {
            io::stdin().read_line(&mut buf).unwrap();
            let action_count = parse_input!(buf, usize);
            for _ in 0..action_count {
                go_to_dustbox(&mut buf);
            }
        }
        println!("{}", u64_to_pos(board.ai()));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Player {
    Black,
    White,
}

#[allow(dead_code)]
impl Player {
    fn opponent(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    fn display(&self) -> &'static str {
        match self {
            Self::Black => "1",
            Self::White => "0",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct Board {
    pub player: u64,
    pub opponent: u64,
    pub turn: Player,
    pub progress: usize,
}

#[allow(dead_code)]
fn pos_to_u64(x: usize, y: usize) -> u64 {
    1 << (x + y * 8)
}

fn u64_to_pos(pos: u64) -> String {
    let x = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let y = ["1", "2", "3", "4", "5", "6", "7", "8"];
    for i in 0..64 {
        if pos & (1 << i) != 0 {
            return format!("{}{}", x[i % 8], y[i / 8]);
        }
    }
    panic!("error");
}

#[allow(dead_code)]
impl Board {
    fn new() -> Self {
        Self {
            player: 0x0000000810000000,
            opponent: 0x0000001008000000,
            turn: Player::Black,
            progress: 4,
        }
    }

    fn from_stdin(turn: Player) -> Self {
        let mut line = String::with_capacity(20);
        let put = 1u64 << 63;
        let mut black = 0u64;
        let mut white = 0u64;
        for _ in 0..8 {
            io::stdin().read_line(&mut line).unwrap();
            for (c, _) in line.chars().zip(0..8) {
                black >>= 1;
                white >>= 1;
                match c {
                    '0' => {
                        black |= put;
                    }
                    '1' => {
                        white |= put;
                    }
                    _ => {}
                }
            }
            line.clear();
        }
        if turn == Player::White {
            (black, white) = (white, black);
        }
        Self {
            player: black,
            opponent: white,
            turn,
            progress: (black.count_ones() + white.count_ones()) as usize,
        }
    }

    fn next(&mut self) {
        self.turn = self.turn.opponent();
        (self.player, self.opponent) = (self.opponent, self.player);
        self.progress += 1;
    }

    fn valid_moves(&self) -> u64 {
        let blank = !(self.player | self.opponent);
        let mut res = 0;

        let mask = HORIZONTAL & self.opponent;
        check_direction!(mask, self.player,  << 1, res , blank); //左
        check_direction!(mask, self.player,  >> 1, res  , blank); //右

        let mask = VERTICAL & self.opponent;
        check_direction!(mask, self.player, << 8, res , blank); //上
        check_direction!(mask, self.player, >> 8, res, blank); //下

        let mask = DIAGONAL & self.opponent;
        check_direction!(mask, self.player,  >> 9, res, blank); //右上
        check_direction!(mask, self.player,  >> 7, res, blank); //右下
        check_direction!(mask, self.player,  << 7, res, blank); //左上
        check_direction!(mask, self.player,  << 9, res, blank); //左下

        res
    }

    fn put(&mut self, pos: u64) {
        let mask = self.opponent & HORIZONTAL;

        let mut reverse: u64 = 0;
        check_reverse!(mask, pos, self.player, << 1, reverse);
        check_reverse!(mask, pos, self.player, >> 1, reverse);

        let mask = self.opponent & VERTICAL;
        check_reverse!(mask, pos, self.player, << 8, reverse);
        check_reverse!(mask, pos, self.player, >> 8, reverse);

        let mask = self.opponent & DIAGONAL;
        check_reverse!(mask, pos, self.player, << 9, reverse);
        check_reverse!(mask, pos, self.player, >> 9, reverse);
        check_reverse!(mask, pos, self.player, << 7, reverse);
        check_reverse!(mask, pos, self.player, >> 7, reverse);

        self.player ^= pos | reverse;
        self.opponent ^= reverse;
    }

    fn score(&self) -> isize {
        (self.player.count_ones() - self.opponent.count_ones()) as isize
    }

    fn ai(&self) -> u64 {
        let mut max_score = -64;
        let valid_moves = self.valid_moves();
        let mut best_move = 0;
        for i in 0..64 {
            let pos = 1 << i;
            if valid_moves & pos != 0 {
                let mut board = (*self).clone();
                board.put(pos);
                let score = board.score();
                if score > max_score {
                    max_score = score;
                    best_move = pos;
                }
            }
        }
        best_move
    }
}
