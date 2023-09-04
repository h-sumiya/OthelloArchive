use crate::mask::POSES; //python:del
use crate::pos::Pos; //python:del
use crate::time::TimeManager; //python:del
use crate::Board; //python:del

const MIN: isize = -10000;
const MAX: isize = 10000;

macro_rules! arufa_beta {
    ($self:expr,$depth:expr,$func:tt) => {{
        let moves = unsafe { $self.legal_moves() };
        let mut scores = vec![];
        for pos in POSES {
            if moves & pos != 0 {
                let board = unsafe { $self.put(Pos(pos)) };
                let score = -board.$func(2, MIN, MAX);
                scores.push((score, pos));
            }
        }
        scores.sort_by_key(|x| -x.0);
        let mut max = MIN;
        let mut max_pos = 0;
        for (_, pos) in scores {
            let board = unsafe { $self.put(Pos(pos)) };
            let score = -board.$func($depth - 1, MIN, -max);
            if score > max {
                max = score;
                max_pos = pos;
            }
        }
        Pos(max_pos)
    }};
}

macro_rules! arufa_beta_node {
    ($self:expr, $depth:expr,$a:expr, $b:expr,$func:tt,$score_func:tt) => {{
        if $depth == 0 {
            return $self.$score_func() as isize;
        }
        let mut moves = unsafe { $self.legal_moves() };
        if moves == 0 {
            moves = unsafe { $self.opp_legal_moves() };
            if moves == 0 {
                return $self.$score_func() as isize;
            }
            return -$self.pass().$func($depth - 1, -$b, -$a);
        }

        for pos in POSES {
            if moves & pos != 0 {
                let board = unsafe { $self.put(Pos(pos)) };
                let score = -board.$func($depth - 1, -$b, -$a);
                $a = $a.max(score);
                if $a >= $b {
                    return $a;
                }
            }
        }
        $a
    }};
}

impl Board {
    pub fn ai2(&self) -> Pos {
        println!("======ai=================");
        eprintln!("score: {}", self._score()); //python:debug
        let remain = self.remain();
        let mut tm = TimeManager::new(); //python:debug
        let pos;
        if remain > 15 {
            eprintln!("mode score"); //python:debug
            pos = self.ab_score(7);
            tm.lap(); //python:debug
        } else if remain > 13 {
            eprintln!("mode win"); //python:debug
            pos = self.ab_win(remain);
            tm.lap(); //python:debug
        } else {
            eprintln!("mode kn"); //python:debug
            pos = self.ab_kn(remain);
            tm.lap(); //python:debug
        }
        pos
    }

    pub fn _score(&self) -> isize {
        self.cn() * 20 + self.bp() * 2 + self.fs() * 75
    }

    pub fn ab_score(&self, depth: usize) -> Pos {
        arufa_beta!(self, depth, ab_score_node)
    }

    pub fn ab_score_node(&self, depth: usize, mut a: isize, b: isize) -> isize {
        arufa_beta_node!(self, depth, a, b, ab_score_node, _score)
    }

    pub fn ab_kn(&self, depth: usize) -> Pos {
        arufa_beta!(self, depth, ab_kn_node)
    }

    pub fn ab_kn_node(&self, depth: usize, mut a: isize, b: isize) -> isize {
        arufa_beta_node!(self, depth, a, b, ab_kn_node, kn)
    }

    pub fn ab_win(&self, depth: usize) -> Pos {
        arufa_beta!(self, depth, ab_win_node)
    }

    pub fn ab_win_node(&self, depth: usize, mut a: isize, b: isize) -> isize {
        arufa_beta_node!(self, depth, a, b, ab_win_node, win)
    }
}
