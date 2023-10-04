use crate::book::KILLER; //python:del
use crate::calc::set_default; //python:del
use crate::pos::Pos; //python:del
use crate::time::TimeManager; //python:del
use crate::Board; //python:del
const MIN: f32 = -10000.0;

struct SortedPos(Vec<(f32, Pos)>);
static mut TIMER: TimeManager = TimeManager::temp();

impl SortedPos {
    fn sort(&mut self) {
        self.0.sort_by_key(|x| (-x.0 * 1000.0) as isize);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win {
    Win,
    Lose,
    Draw,
}

impl Win {
    fn from_i32(i: i32) -> Self {
        if i > 0 {
            Self::Win
        } else if i < 0 {
            Self::Lose
        } else {
            Self::Draw
        }
    }
    fn toggle(&self) -> Self {
        match self {
            Self::Win => Self::Lose,
            Self::Lose => Self::Win,
            Self::Draw => Self::Draw,
        }
    }
}

impl Board {
    fn sorted_pos(&self) -> SortedPos {
        let moves = unsafe { self.legal_moves() };
        let mut scores = vec![];
        let mut b = MIN;
        let prog = self.prog().max(0);
        if prog < 22 {
            set_default(0);
        } else if prog < 37 {
            set_default(1);
        } else {
            set_default(2);
        }
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = unsafe { -board.score_ab_node(2, MIN, -b) };
                b = b.max(score);
                scores.push((score, Pos(pos)));
            }
        }
        let mut res = SortedPos(scores);
        res.sort();
        res
    }

    fn score_ab(&self, mut depth: usize) -> Pos {
        let mut scores = self.sorted_pos();
        loop {
            let mut b = MIN;
            let mut max_pos = scores.0[0].1;
            let prog = self.prog().max(0) + depth;
            if prog < 25 {
                set_default(0);
            } else if prog < 40 {
                set_default(1);
            } else {
                set_default(2);
            }
            for (i, (tsc, pos)) in scores.0.iter_mut().enumerate() {
                let board = unsafe { self.put(*pos) };
                let score = unsafe { -board.score_ab_node(depth - 1, MIN, -b) };
                *tsc = score;
                if score > b {
                    b = score;
                    max_pos = *pos;
                }
                if unsafe { TIMER.lap().is_err() } {
                    eprintln!("break depth: {}({}/{})", depth, i, scores.0.len()); //python:debug
                    return max_pos;
                }
            }
            if unsafe { TIMER.next().is_err() } {
                return max_pos;
            }
            scores.sort();
            depth += 1;
        }
    }

    unsafe fn score_ab_node(&self, depth: usize, mut a: f32, b: f32) -> f32 {
        if depth == 0 {
            return self.defalut_score();
        }
        let mut moves = unsafe { self.legal_moves() };
        if moves == 0 {
            moves = unsafe { self.opp_legal_moves() };
            if moves == 0 {
                return (self.kn() * 1000) as f32;
            }
            return -self.pass().score_ab_node(depth - 1, -b, -a);
        }

        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = -board.score_ab_node(depth - 1, -b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
                if depth > 5 {
                    if unsafe { TIMER.check().is_err() } {
                        eprintln!("break score node"); //python:debug
                        return a;
                    }
                }
            }
        }
        a
    }

    pub fn win_ab(&self) -> Pos {
        let depth = 64 - self.count();
        let sorted = self.sorted_pos();
        let mut max_pos = sorted.0[0].1;
        let mut max_score = Win::Lose;
        for (_, pos) in sorted.0 {
            let board = unsafe { self.put(pos) };
            let res = board.win_ab_node(depth - 1).toggle();
            if res == Win::Win {
                return pos;
            } else if res == Win::Draw && max_score == Win::Lose {
                max_pos = pos;
                max_score = Win::Draw;
            }
            if unsafe { TIMER.lap() }.is_err() {
                eprintln!("break win"); //python:debug
                return max_pos;
            }
        }
        max_pos
    }

    pub fn win_ab_node(&self, depth: usize) -> Win {
        if depth == 1 {
            let pos = unsafe { self.legal_moves() };
            let socre;
            if pos != 0 {
                socre = Win::from_i32(unsafe { self.last_kn(Pos(pos)) });
            } else {
                let pos = unsafe { self.opp_legal_moves() };
                if pos != 0 {
                    let s = unsafe { self.pass().last_kn(Pos(pos)) };
                    socre = Win::from_i32(s).toggle();
                } else {
                    socre = Win::from_i32(self.kn());
                }
            }
            return socre;
        }
        let mut moves = unsafe { self.legal_moves() };
        if moves == 0 {
            moves = unsafe { self.opp_legal_moves() };
            if moves == 0 {
                return Win::from_i32(self.kn());
            }
            return self.pass().win_ab_node(depth - 1).toggle();
        }
        let mut score = Win::Lose;
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let res = board.win_ab_node(depth - 1).toggle();
                if res == Win::Win {
                    return Win::Win;
                } else if res == Win::Draw {
                    score = Win::Draw;
                }
            }
        }
        score
    }

    pub fn kn_ab(&self) -> Pos {
        let sorted = self.sorted_pos();
        let mut max_pos = sorted.0[0].1;
        let mut max_score = -10000;
        let depth = 64 - self.count();
        for (_, pos) in sorted.0 {
            let board = unsafe { self.put(pos) };
            let res = -board.kn_ab_node(depth - 1, -10000, -max_score);
            if res > max_score {
                max_pos = pos;
                max_score = res;
            }
            if unsafe { TIMER.lap() }.is_err() {
                eprintln!("break kn"); //python:debug
                return max_pos;
            }
        }
        max_pos
    }

    pub fn kn_ab_node(&self, depth: usize, mut a: i32, b: i32) -> i32 {
        if depth == 1 {
            let pos = unsafe { self.legal_moves() };
            let socre;
            if pos != 0 {
                socre = unsafe { self.last_kn(Pos(pos)) };
            } else {
                let pos = unsafe { self.opp_legal_moves() };
                if pos != 0 {
                    let s = -unsafe { self.pass().last_kn(Pos(pos)) };
                    socre = s;
                } else {
                    socre = self.kn();
                }
            }
            return socre;
        }
        let mut moves = unsafe { self.legal_moves() };
        if moves == 0 {
            moves = unsafe { self.opp_legal_moves() };
            if moves == 0 {
                return self.kn();
            }
            return -self.pass().kn_ab_node(depth - 1, -b, -a);
        }
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = -board.kn_ab_node(depth - 1, -b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
            }
        }
        a
    }

    pub fn simple_kn_ab(&self) -> Pos {
        let moves = unsafe { self.legal_moves() };
        let mut max_score = -10000;
        let mut max_pos = Pos(0);
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = -board.simple_kn_ab_node();
                if score > max_score {
                    max_score = score;
                    max_pos = Pos(pos);
                }
            }
        }
        max_pos
    }

    pub fn simple_kn_ab_node(&self) -> i32 {
        let moves = unsafe { self.legal_moves() };
        if moves == 0 {
            let moves = unsafe { self.opp_legal_moves() };
            if moves == 0 {
                return self.kn();
            }
            return -self.pass().simple_kn_ab_node();
        }
        let mut max_score = -10000;
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = -board.simple_kn_ab_node();
                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }

    pub fn ai(&self) -> Pos {
        eprintln!("score: {}", unsafe { self.score() }); //python:debug
        let remain = self.remain();
        unsafe { TIMER.start() };
        let pos;
        if let Some(p) = unsafe { KILLER.get(&(self.me, self.opp)) } {
            eprintln!("mode killer");
            pos = *p;
        } else if remain <= 5 {
            eprintln!("mode simple kn");
            pos = self.simple_kn_ab();
        } else if remain <= 13 {
            eprintln!("mode kn");
            pos = self.kn_ab();
        } else if remain <= 15 {
            eprintln!("mode win");
            pos = self.win_ab();
        } else {
            eprintln!("mode socre");
            pos = self.score_ab(6);
        }
        unsafe { TIMER.finish() };
        pos
    }
}
