use crate::mask::*; //python:del
use crate::pos::Pos; //python:del
use std::arch::x86_64::*; //python:del
use std::mem::transmute; //python:del

//参考
//https://github.com/okuhara/edax-reversi-AVX/

const PPMASK: ([__m256i; 66], [__m256i; 66]) = include_data!(4224, "ヾ　　　㄀㄁㄁㄁㈀㠄倐끀　　　　ー　　　㈀㈂㈂㈂㐀䀈瀠む㄀　　　ヸ　　　㐀㐄㐄㐄㠀倐끀　㈀、　　ヰ　　　㠀㠈㠈㠈䀀瀠む　㐀㄂　　ム　　　䀀䀐䀐䀐倀끀　　㠀㈄、　ダ　　　倀倠倠倠瀀む　　䀀㐈㄂　む　　　瀀灀灀灀뀀　　　倀㠐㈄、　　　　뀀낀낀낀　　　　瀀䀠㐈㄂񃸀　　　　㄁㄁㄁　㐂䀈瀠　　　　񃰀　　　　㈂㈂㈂　㠄倐끀　、　　񃠀　　　　㐄㐄㐄　䀈瀠む　㄂　　񃀀　　　　㠈㠈㠈　倐끀　　㈄、　񂀀　　　　䀐䀐䀐　瀠む　　㐈㄂　񀀀　　　　倠倠倠　끀　　　㠐㈄、뀀　　　　灀灀灀　む　　　䀠㐈㄂　　　　　낀낀낀　　　　　偀㠐㈄　ヾ　　　㄀㄁㄁　㈀㠄倐　　　　　ー　　　㈀㈂㈂　㐀䀈瀠　㄀　　　ヸ　　　㐀㐄㐄　㠀倐끀　㈀、　　ヰ　　　㠀㠈㠈　䀀瀠む　㐀㄂　　ム　　　䀀䀐䀐　倀끀　　㠀㈄、　ダ　　　倀倠倠　瀀む　　䀀㐈㄂　む　　　瀀灀灀　뀀　　　倀㠐㈄　　　　　뀀낀낀　　　　　瀀䀠㐈　񃸀　　　　㄁㄁　　㐂䀈　　　　　񃰀　　　　㈂㈂　　㠄倐　　、　　񃠀　　　　㐄㐄　　䀈瀠　　㄂　　񃀀　　　　㠈㠈　　倐끀　　㈄、　񂀀　　　　䀐䀐　　瀠む　　㐈㄂　񀀀　　　　倠倠　　끀　　　㠐㈄　뀀　　　　灀灀　　む　　　䀠㐈　　　　　　낀낀　　　　　　偀㠐　　ヾ　　　㄀㄁　　㈀㠄　　　　　　ー　　　㈀㈂　　㐀䀈　　㄀　　　ヸ　　　㐀㐄　　㠀倐　　㈀、　　ヰ　　　㠀㠈　　䀀瀠　　㐀㄂　　ム　　　䀀䀐　　倀끀　　㠀㈄　　ダ　　　倀倠　　瀀む　　䀀㐈　　む　　　瀀灀　　뀀　　　倀㠐　　　　　　뀀낀　　　　　　瀀䀠　　񃸀　　　　㄁　　　㐂　　　　　　񃰀　　　　㈂　　　㠄　　　、　　񃠀　　　　㐄　　　䀈　　　㄂　　񃀀　　　　㠈　　　倐　　　㈄　　񂀀　　　　䀐　　　瀠　　　㐈　　񀀀　　　　倠　　　끀　　　㠐　　뀀　　　　灀　　　む　　　䀠　　　　　　　낀　　　　　　　偀　　　ヾ　　　㄀　　　㈀　　　　　　　ー　　　㈀　　　㐀　　　㄀　　　ヸ　　　㐀　　　㠀　　　㈀　　　ヰ　　　㠀　　　䀀　　　㐀　　　ム　　　䀀　　　倀　　　㠀　　　ダ　　　倀　　　瀀　　　䀀　　　む　　　瀀　　　뀀　　　倀　　　　　　　뀀　　　　　　　瀀　　　񃸀　　　　　　　　　　　　　　　񃰀　　　　　　　　　　　　　　　񃠀　　　　　　　　　　　　　　　񃀀　　　　　　　　　　　　　　　񂀀　　　　　　　　　　　　　　　񀀀　　　　　　　　　　　　　　　뀀　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　、　　　　　　　　　　　　　　　〃　　　　　　　　　　　　　　　〇　　　　　　　　　　　　　　　』　　　　　　　　　　　　　　　〟　　　　　　　　　　　　　　　〿　　　　　　　　　　　　　　　み　　　　　　　　　　　　　　　　　　　、　　　　　　　。　　　㄀　　　。　　　、　　　〄　　　㌀　　　〄　　　。　　　〈　　　㜀　　　〈　　　〄　　　【　　　㼀　　　【　　　〈　　　〠　　　伀　　　〠　　　【　　　぀　　　漀　　　぀　　　〠　　　む　　　꼀　　　む　　　぀　　　　　　　　　　　㄁　　　　　　　㈄　　　　、　　㈂　　　㄀　　　㐈　　　　〃　　㐄　　　㈁　　　㠐　　　　〇　　㠈　　　㐂　　　䀠　　　　』　　䀐　　　㠄　　　偀　　　　〟　　倠　　　䀈　　　炀　　　　〿　　灀　　　倐　　　뀀　　　　み　　낀　　　瀠　　　　　　　　　　　㄁、　　　　　　㐈。　　　㄀　　㈂。　　　、　　㠐〄　　　㌀　　㐄〄　　㄀。　　䀠〈　　　㜀　　㠈〈　　㈁〄　　偀【　　　㼀　　䀐【　　㐂〈　　炀〠　　　伀　　倠〠　　㠄【　　뀀぀　　　漀　　灀぀　　䀈〠　　　む　　　꼀　　낀む　　倐぀　　　　　　　　　　㄁㄁　　　　　　㠐㈄　　　　、　㈂㈂　　　㄀　　䀠㐈　　　　〃　㐄㐄　　　㈁　　偀㠐　　　　〇　㠈㠈　　㄀㐂　　炀䀠　　　　』　䀐䀐　　㈁㠄　　뀀偀　　　　〟　倠倠　　㐂䀈　　　炀　　　　〿　灀灀　　㠄倐　　　뀀　　　　み　낀낀　　䀈瀠　　　　　　　　　　㄁㄁、　　　　　䀠㐈。　　　㄀　㈂㈂。　　　、　偀㠐〄　　　㌀　㐄㐄〄　　㄀。　炀䀠〈　　　㜀　㠈㠈〈　　㈁〄　뀀偀【　　　㼀　䀐䀐【　㄀㐂〈　　炀〠　　　伀　倠倠〠　㈁㠄【　　뀀぀　　　漀　灀灀぀　㐂䀈〠　　　む　　　꼀　낀낀む　㠄倐぀　　　　　　　　　㄁㄁㄁　　　　　偀㠐㈄　　　　、㈂㈂㈂　　　㄀　炀䀠㐈　　　　〃㐄㐄㐄　　　㈁　뀀偀㠐　　　　〇㠈㠈㠈　　㄀㐂　　炀䀠　　　　』䀐䀐䀐　　㈁㠄　　뀀偀　　　　〟倠倠倠　㄀㐂䀈　　　炀　　　　〿灀灀灀　㈁㠄倐　　　뀀　　　　み낀낀낀　㐂䀈瀠　　　　　　　　　㄁㄁㄁、　　　　炀䀠㐈。　　　㄀㈂㈂㈂。　　　、뀀偀㠐〄　　　㌀㐄㐄㐄〄　　㄀。　炀䀠〈　　　㜀㠈㠈㠈〈　　㈁〄　뀀偀【　　　㼀䀐䀐䀐【　㄀㐂〈　　炀〠　　　伀倠倠倠〠　㈁㠄【　　뀀぀　　　漀灀灀灀぀㄀㐂䀈〠　　　む　　　꼀낀낀낀む㈁㠄倐぀　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　");
const SHIFT1897: __m256i = unsafe { transmute([7u64, 9, 8, 1]) };
const MFLIP_H: __m256i = include_data!(32, "깾깾깾깾깾깾깾깾񃿿񃿿񃿿񃿿깾깾깾깾");
const MM7981: __m256i = unsafe { transmute([1i64, 8, 9, 7]) };
const MM1418: __m256i = unsafe { transmute([2i64, 16, 18, 14]) };
const MM2836: __m256i = unsafe { transmute([4i64, 32, 36, 28]) };
const MM0000: __m256i = unsafe { transmute([0i64, 0, 0, 0]) };

pub struct Board {
    pub me: u64,
    pub opp: u64,
}

const LAST_BIT: u64 = POSES[63];
impl Board {
    pub fn new() -> Self {
        Self {
            me: 0x0000000810000000,
            opp: 0x0000001008000000,
        }
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

    #[inline]
    pub fn blank(&self) -> u64 {
        !(self.me | self.opp)
    }

    #[inline]
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

    #[inline]
    #[target_feature(enable = "avx2")]
    pub unsafe fn legal_moves(&self) -> u64 {
        let pp = _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.me)));
        let moo = _mm256_and_si256(
            _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.opp))),
            MFLIP_H,
        );
        Self::legal_moves_base(pp, moo) & self.blank()
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    pub unsafe fn opp_legal_moves(&self) -> u64 {
        let pp = _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.opp)));
        let moo = _mm256_and_si256(
            _mm256_broadcastq_epi64(_mm_cvtsi64_si128(transmute(self.me))),
            MFLIP_H,
        );
        Self::legal_moves_base(pp, moo) & self.blank()
    }

    #[inline]
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
