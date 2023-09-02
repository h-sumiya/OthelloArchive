use crate::calc2::set_default_score; //python:del
use crate::pos::Pos; //python:del
use crate::time::TimeManager; //python:del
use crate::Board; //python:del

const MIN: f32 = -10000.0;

impl Board {
    fn score_ab(&self, depth: usize) -> Pos {
        let moves = unsafe { self.legal_moves() };
        let mut scores = vec![];
        let mut b = MIN;
        set_default_score(self.prog() + 3);
        for i in 0..64 {
            let pos = 1 << i;
            if moves & pos != 0 {
                let board = unsafe { self.put(Pos(pos)) };
                let score = unsafe { -board.score_ab_node(2, MIN, -b) };
                b = b.max(score);
                scores.push((score, pos));
            }
        }
        scores.sort_by_key(|x| (-x.0 * 1000.0) as isize);
        b = MIN;
        let mut max_pos = 0;
        set_default_score(self.prog() + depth);
        for (_, pos) in scores {
            let board = unsafe { self.put(Pos(pos)) };
            let score = unsafe { -board.score_ab_node(depth - 1, MIN, -b) };
            if score > b {
                b = score;
                max_pos = pos;
            }
        }
        Pos(max_pos)
    }

    unsafe fn score_ab_node(&self, depth: usize, mut a: f32, b: f32) -> f32 {
        if depth == 0 {
            return self.defalut_score();
        }
        let mut moves = unsafe { self.legal_moves() };
        if moves == 0 {
            moves = unsafe { self.opp_legal_moves() };
            if moves == 0 {
                return self._score();
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
            }
        }
        a
    }

    pub fn ai2(&self) -> Pos {
        println!("======ai2=================");
        eprintln!("score: {}", unsafe { self.defalut_score() }); //python:debug
        let remain = self.remain();
        let mut tm = TimeManager::new(); //python:debug
        let pos;
        if remain > 15 {
            eprintln!("mode score"); //python:debug
            pos = self.score_ab(7);
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
}
