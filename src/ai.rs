use crate::mask::POSES; //python:del
use crate::pos::Pos; //python:del
use crate::time::TimeManager; //python:del
use crate::Board; //python:del

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
