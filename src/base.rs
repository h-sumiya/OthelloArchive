use crate::mask::PPMASK;
use crate::pos::Pos;
use std::arch::x86_64::*;
use std::mem::transmute;

const SHIFT1897: __m256i = unsafe { transmute([7u64, 9, 8, 1]) };
const MFLIP_H: __m256i = unsafe {
    transmute([
        9114861777597660798i64,
        9114861777597660798,
        -1,
        9114861777597660798,
    ])
};
const MM7981: __m256i = unsafe { transmute([1i64, 8, 9, 7]) };
const MM1418: __m256i = unsafe { transmute([2i64, 16, 18, 14]) };
const MM2836: __m256i = unsafe { transmute([4i64, 32, 36, 28]) };
const MM0000: __m256i = unsafe { transmute([0i64, 0, 0, 0]) };

pub struct Board {
    pub me: u64,
    pub opp: u64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            me: 0x0000000810000000,
            opp: 0x0000001008000000,
        }
    }

    #[inline]
    pub fn blank(&self) -> u64 {
        !(self.me | self.opp)
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn legal_moves_base(pp: __m256i, moo: __m256i) -> u64 {
        let mut flip_l: __m256i;
        let mut flip_r: __m256i;
        let pre_l: __m256i;
        let pre_r: __m256i;
        let shift2: __m256i;
        let mut mm: __m256i;
        let mut m: __m128i;
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
        transmute::<i64, u64>(_mm_cvtsi128_si64(m))
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn legal_moves(&self) -> u64 {
        let pp = _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.me)));
        let moo = _mm256_and_si256(
            _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.opp))),
            MFLIP_H,
        );
        Self::legal_moves_base(pp, moo) & self.blank()
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn opp_legal_moves(&self) -> u64 {
        let pp = _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.opp)));
        let moo = _mm256_and_si256(
            _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.me))),
            MFLIP_H,
        );
        Self::legal_moves_base(pp, moo) & self.blank()
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn put(&self, pos: Pos) -> Self {
        let i = pos.idx();
        let pp: __m256i;
        let oo: __m256i;
        let mut flip: __m256i;
        let mut outflank: __m256i;
        let mut eraser: __m256i;
        let mut mask: __m256i;
        let mut flip2: __m128i;
        let me: u64;
        let opp: u64;
        pp = _mm256_set1_epi64x(transmute(self.me));
        oo = _mm256_set1_epi64x(transmute(self.opp));
        mask = PPMASK.1[i];
        eraser = _mm256_andnot_si256(oo, mask);
        outflank = _mm256_sllv_epi64(_mm256_and_si256(pp, mask), MM7981);
        eraser = _mm256_or_si256(eraser, _mm256_srlv_epi64(eraser, MM7981));
        outflank = _mm256_andnot_si256(eraser, outflank);
        outflank = _mm256_andnot_si256(_mm256_srlv_epi64(eraser, MM1418), outflank);
        outflank = _mm256_andnot_si256(_mm256_srlv_epi64(eraser, MM2836), outflank);
        flip = _mm256_and_si256(mask, _mm256_sub_epi64(_mm256_setzero_si256(), outflank));
        mask = PPMASK.0[i];
        outflank = _mm256_andnot_si256(oo, mask);
        outflank = _mm256_and_si256(outflank, _mm256_sub_epi64(MM0000, outflank));
        outflank = _mm256_and_si256(outflank, pp);
        eraser = _mm256_sub_epi64(
            _mm256_cmpeq_epi64(outflank, _mm256_setzero_si256()),
            outflank,
        );
        flip = _mm256_or_si256(flip, _mm256_andnot_si256(eraser, mask));
        flip2 = _mm_or_si128(
            _mm256_castsi256_si128(flip),
            _mm256_extracti128_si256(flip, 1),
        );
        flip2 = _mm_or_si128(flip2, _mm_shuffle_epi32(flip2, 0x4e));
        (me, opp) = transmute(_mm_xor_si128(
            _mm_set_epi64x(transmute(self.me), transmute(self.opp)),
            flip2,
        ));
        Self {
            me,
            opp: opp | pos.0,
        }
    }

    pub fn pass(&self) -> Self {
        Self {
            me: self.opp,
            opp: self.me,
        }
    }
}
