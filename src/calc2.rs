use crate::base::Board; //python:del
use crate::bin::SCORE_DATA; //python:del
use crate::score::{Index, Score, INDEX_TABLE}; //python:del
use std::mem::transmute; //python:del

pub static mut SCORE_TABLE: [Score; 64] = unsafe { transmute([1u8; 5120]) };
pub static mut DEFALUT_SCORE: &'static Score = unsafe { &SCORE_TABLE[0] };

pub fn first_load() {
    let locs: [usize; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 12, 12, 12, 12,
        13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
        13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13
    ];
    unsafe {
        for i in 0..64 {
            let loc = locs[i] * 617;
            let mut s = Score::load(&SCORE_DATA[loc..]);
            std::mem::swap(&mut SCORE_TABLE[i], &mut s);
            std::mem::forget(s);
        }
    }
}

pub fn set_default_score(index: usize) {
    unsafe {
        DEFALUT_SCORE = &SCORE_TABLE[index];
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

    pub fn cn(&self) -> isize {
        unsafe {
            let me = self.legal_moves().count_ones() as isize;
            let opp = self.opp_legal_moves().count_ones() as isize;
            me - opp
        }
    }
    #[target_feature(enable = "avx2")]
    unsafe fn score_index(&self) -> Index {
        let mut index: Index;
        let opp_index: [u8; 8] = transmute(self.opp);
        index = INDEX_TABLE[0][opp_index[0] as usize];
        for i in 1..8 {
            index += INDEX_TABLE[i][opp_index[i] as usize];
        }
        index = index.x2();
        let me_index: [u8; 8] = transmute(self.me);
        for i in 0..8 {
            index += INDEX_TABLE[i][me_index[i] as usize];
        }
        index
    }
    #[target_feature(enable = "avx2")]
    pub unsafe fn defalut_score(&self) -> f32 {
        let index = self.score_index();
        let cn = self.cn();
        DEFALUT_SCORE.calc(index, cn)
    }
    #[target_feature(enable = "avx2")]
    pub unsafe fn _score(&self) -> f32 {
        let index = self.score_index();
        let cn = self.cn();
        SCORE_TABLE[self.count() - 4].calc(index, cn)
    }
}
