use crate::mask::*; //python:del
use crate::pos::Pos; //python:del
use std::arch::x86_64::*;
use std::mem::transmute;

const SHIFT1897: __m256i = unsafe { transmute([7u64, 9u64, 8u64, 1u64]) };
const MFLIP_H: __m256i = unsafe {
    transmute([
        0x7e7e7e7e7e7e7e7e,
        0x7e7e7e7e7e7e7e7e,
        -1i64,
        0x7e7e7e7e7e7e7e7e,
    ])
};

fn get_moves(P: u64, O: u64) -> u64 {
    let mut moves: u64;
    let mut mO: u64;
    let mut flip1: u64;
    let mut pre1: u64;
    let mut flip8: u64;
    let mut pre8: u64;
    let mut PP: __m128i;
    let mut mOO: __m128i;
    let mut MM: __m128i;
    let mut flip: __m128i;
    let mut pre: __m128i;

    mO = O & 0x7e7e7e7e7e7e7e7e;
    PP = unsafe { _mm_set_epi64x(transmute(vertical_mirror(P)), transmute(P)) };
    mOO = unsafe { _mm_set_epi64x(transmute(vertical_mirror(mO)), transmute(mO)) };

    flip = unsafe { _mm_and_si128(mOO, _mm_slli_epi64(PP, 7)) };
    flip1 = mO & (P << 1);
    flip8 = O & (P << 8);
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(mOO, _mm_slli_epi64(flip, 7))) };
    flip1 |= mO & (flip1 << 1);
    flip8 |= O & (flip8 << 8);
    pre = unsafe { _mm_and_si128(mOO, _mm_slli_epi64(mOO, 7)) };
    pre1 = mO & (mO << 1);
    pre8 = O & (O << 8);
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 14))) };
    flip1 |= pre1 & (flip1 << 2);
    flip8 |= pre8 & (flip8 << 16);
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 14))) };
    flip1 |= pre1 & (flip1 << 2);
    flip8 |= pre8 & (flip8 << 16);
    MM = unsafe { _mm_slli_epi64(flip, 7) };
    moves = flip1 << 1;
    moves |= flip8 << 8;

    flip = unsafe { _mm_and_si128(mOO, _mm_slli_epi64(PP, 9)) };
    flip1 = mO & (P >> 1);
    flip8 = O & (P >> 8);
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(mOO, _mm_slli_epi64(flip, 9))) };
    flip1 |= mO & (flip1 >> 1);
    flip8 |= O & (flip8 >> 8);
    pre = unsafe { _mm_and_si128(mOO, _mm_slli_epi64(mOO, 9)) };
    pre1 >>= 1;
    pre8 >>= 8;
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 18))) };
    flip1 |= pre1 & (flip1 >> 2);
    flip8 |= pre8 & (flip8 >> 16);
    flip = unsafe { _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 18))) };
    flip1 |= pre1 & (flip1 >> 2);
    flip8 |= pre8 & (flip8 >> 16);
    MM = unsafe { _mm_or_si128(MM, _mm_slli_epi64(flip, 9)) };
    moves |= flip1 >> 1;
    moves |= flip8 >> 8;

    moves |= unsafe { transmute::<_, u64>(_mm_cvtsi128_si64(MM)) }
        | vertical_mirror(unsafe { transmute(_mm_cvtsi128_si64(_mm_unpackhi_epi64(MM, MM))) });

    moves & !(P | O)
}

const mask0F0F: __m128i = unsafe {
    transmute([
        0x0F0Fi16, 0x0F0F, 0x0F0F, 0x0F0F, 0x0F0F, 0x0F0F, 0x0F0F, 0x0F0F,
    ])
};
const mbitrev: __m128i =
    unsafe { transmute([15i8, 7, 11, 3, 13, 5, 9, 1, 14, 6, 10, 2, 12, 4, 8, 0]) };

fn vertical_mirror(x: u64) -> u64 {
    let bb: __m128i = unsafe {
        let bb = _mm_set_epi64x(x as i64, 0);
        let shifted = _mm_srli_epi64(bb, 4);
        let shuffled1 = _mm_shuffle_epi8(mbitrev, _mm_and_si128(shifted, mask0F0F));
        let shuffled2 = _mm_shuffle_epi8(mbitrev, _mm_and_si128(bb, mask0F0F));
        let shifted2 = _mm_slli_epi64(shuffled2, 4);
        _mm_or_si128(shuffled1, shifted2)
    };
    unsafe { _mm_cvtsi128_si64(bb) as u64 }
}

pub struct Boardv2 {
    pub me: u64,
    pub opp: u64,
}

const LAST_BIT: u64 = POSES[63];
impl Boardv2 {
    pub fn new() -> Self {
        Self { me: 0, opp: 0 }
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

    pub fn legal_moves(&self) -> u64 {
        get_moves(self.me, self.opp)
    }

    pub fn legal_moves_(&self) -> u64 {
        unsafe {
            let mut flip_l: __m256i;
            let mut flip_r: __m256i;
            let pre_l: __m256i;
            let pre_r: __m256i;
            let shift2: __m256i;
            let mut mm: __m256i;
            let mut m: __m128i;
            let pp = _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.me)));
            let moo = _mm256_and_si256(
                _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.opp))),
                MFLIP_H,
            );

            flip_l = _mm256_and_si256(moo, _mm256_sllv_epi64(pp, SHIFT1897));
            flip_r = _mm256_and_si256(moo, _mm256_srlv_epi64(pp, SHIFT1897));
            flip_l = _mm256_or_si256(
                flip_l,
                _mm256_and_si256(moo, _mm256_sllv_epi64(flip_l, SHIFT1897)),
            );
            flip_r = _mm256_or_si256(
                flip_r,
                _mm256_and_si256(moo, _mm256_srlv_epi64(flip_r, SHIFT1897)),
            );
            pre_l = _mm256_and_si256(moo, _mm256_sllv_epi64(moo, SHIFT1897));
            pre_r = _mm256_srlv_epi64(pre_l, SHIFT1897);
            shift2 = _mm256_add_epi64(SHIFT1897, SHIFT1897);
            flip_l = _mm256_or_si256(
                flip_l,
                _mm256_and_si256(pre_l, _mm256_sllv_epi64(flip_l, shift2)),
            );
            flip_r = _mm256_or_si256(
                flip_r,
                _mm256_and_si256(pre_r, _mm256_srlv_epi64(flip_r, shift2)),
            );
            flip_l = _mm256_or_si256(
                flip_l,
                _mm256_and_si256(pre_l, _mm256_sllv_epi64(flip_l, shift2)),
            );
            flip_r = _mm256_or_si256(
                flip_r,
                _mm256_and_si256(pre_r, _mm256_srlv_epi64(flip_r, shift2)),
            );
            mm = _mm256_sllv_epi64(flip_l, SHIFT1897);
            mm = _mm256_or_si256(mm, _mm256_srlv_epi64(flip_r, SHIFT1897));

            m = _mm_or_si128(_mm256_castsi256_si128(mm), _mm256_extracti128_si256(mm, 1));
            m = _mm_or_si128(m, _mm_unpackhi_epi64(m, m));
            transmute::<i64, u64>(_mm_cvtsi128_si64(m)) & !(self.me | self.opp)
        }
    }

    pub fn opp_legal_moves(&self) -> u64 {
        0
    }

    pub fn put(&self, pos: Pos) -> Self {
        Self {
            me: self.opp,
            opp: self.me,
        }
    }

    pub fn pass(&self) -> Self {
        Self {
            me: self.opp,
            opp: self.me,
        }
    }
}
