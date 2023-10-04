use crate::base::Board;
use crate::score::{Index, INDEX_TABLE, SCORE_TABLE};
use std::mem::transmute;

impl Board {
    pub fn cn(&self) -> isize {
        let me = self.legal_moves().count_ones() as isize;
        let opp = self.opp_legal_moves().count_ones() as isize;
        me - opp
    }

    fn score_index(&self) -> Index {
        unsafe {
            let mut index: Index;
            let opp_index: [u8; 8] = transmute(self.opp);
            index = INDEX_TABLE[0][opp_index[0] as usize];
            for i in 1..8 {
                index += *INDEX_TABLE
                    .get_unchecked(i)
                    .get_unchecked(opp_index[i] as usize);
            }
            index = index * 2;
            let me_index: [u8; 8] = transmute(self.me);
            for i in 0..8 {
                index += *INDEX_TABLE
                    .get_unchecked(i)
                    .get_unchecked(me_index[i] as usize);
            }
            index
        }
    }

    pub fn score(&self) -> f32 {
        let index = self.score_index();
        let cn = self.cn();
        SCORE_TABLE[self.count].calc(index, cn)
    }

    pub fn kn(&self) -> i32 {
        unsafe {
            let me: i32 = transmute(self.me.count_ones());
            let opp: i32 = transmute(self.opp.count_ones());
            me - opp
        }
    }
}
