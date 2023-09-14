use crate::base::Board; //python:del
use crate::bin::*; //python:del
use crate::pos::Pos; //python:del
use crate::score::Score; //python:del
use std::mem::transmute; //python:del

static mut SCORE0: Score = unsafe { transmute([1u8; 4384]) };
static mut SCORE1: Score = unsafe { transmute([1u8; 4384]) };
static mut SCORE2: Score = unsafe { transmute([1u8; 4384]) };
static mut SCORE_DEFAULT: &Score = unsafe { &SCORE0 };
pub fn init_score() {
    unsafe {
        let mut score0 = Score::new(&E80, &EDGES0, &R_0);
        std::mem::swap(&mut SCORE0, &mut score0);
        std::mem::forget(score0);
        let mut score1 = Score::new(&E81, &EDGES1, &R_1);
        std::mem::swap(&mut SCORE1, &mut score1);
        std::mem::forget(score1);
        let mut score2 = Score::new(&E82, &EDGES2, &R_2);
        std::mem::swap(&mut SCORE2, &mut score2);
        std::mem::forget(score2);

        SCORE_DEFAULT = &SCORE0;
    }
}
pub fn set_default(id: usize) {
    unsafe {
        SCORE_DEFAULT = match id {
            0 => &SCORE0,
            1 => &SCORE1,
            2 => &SCORE2,
            _ => &SCORE0,
        };
    }
}

impl Board {
    pub fn count(&self) -> usize {
        (self.me.count_ones() + self.opp.count_ones()) as usize
    }

    pub fn remain(&self) -> usize {
        64 - self.count()
    }

    pub fn prog(&self) -> usize {
        self.count() - 4
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn defalut_score(&self) -> f32 {
        SCORE_DEFAULT.score(self)
    }

    #[allow(dead_code)]
    #[target_feature(enable = "avx2")]
    pub unsafe fn score(&self) -> f32 {
        let c = self.count();
        if c < 29 {
            return SCORE0.score(self);
        } else if c < 44 {
            return SCORE1.score(self);
        } else {
            return SCORE2.score(self);
        }
    }

    pub fn kn(&self) -> i32 {
        unsafe {
            let me: i32 = transmute(self.me.count_ones());
            let opp: i32 = transmute(self.opp.count_ones());
            me - opp
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn last_kn(&self, pos: Pos) -> i32 {
        let lf = self.count_last(pos);
        let me = self.me.count_ones();
        ((1 + me + lf) as i32 - 32) * 2
    }
}
