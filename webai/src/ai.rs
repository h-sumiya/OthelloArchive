use crate::base::Board;
use crate::book::KILLER;
use crate::pos::Pos;


const MIN: f32 = -10000.0;

struct SortedPos(Vec<(f32, Pos)>);

impl SortedPos {
    fn sort(&mut self) {
        self.0.sort_by_key(|x| (-x.0 * 1000.0) as isize);
    }
}

impl Board {
    fn sorted_pos(&self) -> SortedPos {
        let moves = self.legal_moves();
        let mut scores = vec![];
        let mut b = MIN;
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.score_ab_node(2, MIN, -b) ;
                b = b.max(score);
                scores.push((score, Pos(pos)));
            }
        }
        let mut res = SortedPos(scores);
        res.sort();
        res
    }

    fn score_ab(&self, depth: usize) -> Pos {
        let mut scores = self.sorted_pos();
        let mut b = MIN;
        let mut max_pos = scores.0[0].1;
        for (tsc, pos) in scores.0.iter_mut() {
            let board = self.put(*pos);
            let score = -board.score_ab_node(depth - 1, MIN, -b);
            *tsc = score;
            if score > b {
                b = score;
                max_pos = *pos;
            }
        }
        return max_pos;
    }

    fn score_ab_node(&self, depth: usize, mut a: f32, b: f32) -> f32 {
        if depth == 0 {
            return self.score();
        }
        let mut moves = self.legal_moves();
        if moves == 0 {
            moves = self.opp_legal_moves();
            if moves == 0 {
                return (self.kn() * 1000) as f32;
            }
            return -self.pass().score_ab_node(depth - 1, -b, -a);
        }
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.score_ab_node(depth - 1, -b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
            }
        }
        a
    }

    pub fn kn_ab(&self) -> Pos {
        let mut max_pos = Pos(0);
        let mut max_score = -10000;
        let moves = self.legal_moves();
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let res = -board.kn_ab_node(-10000, -max_score);
                if res > max_score {
                    max_pos = Pos(pos);
                    max_score = res;
                }
            }
        }
        max_pos
    }

    pub fn kn_ab_node(&self, mut a: i32, b: i32) -> i32 {
        if self.count == 60 {
            return self.kn();
        }
        let mut moves = self.legal_moves();
        if moves == 0 {
            moves = self.opp_legal_moves();
            if moves == 0 {
                return self.kn();
            }
            return -self.pass().kn_ab_node(-b, -a);
        }
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = self.put(Pos(pos));
                let score = -board.kn_ab_node(-b, -a);
                a = a.max(score);
                if a >= b {
                    return a;
                }
            }
        }
        a
    }

    pub fn ai(&self) -> Pos {
        let pos;
        let remain = 60 - self.count;
        if let Some(p) = unsafe { KILLER.get(&(self.me, self.opp)) } {
            pos = *p;
        } else if remain <= 16 {
            pos = self.kn_ab();
        } else {
            pos = self.score_ab(8);
        }
        pos
    }
}
