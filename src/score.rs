use crate::base::Board; //python:del
use crate::bin::MASKS; //python:del
use crate::edge8::__m256_hori_sum; //python:del
use crate::edge8::EDGE8; //python:del
use crate::edge8::T24; //python:del
use crate::index::Pattern3; //python:del
use crate::index::Pattern4; //python:del
use crate::index::Pattern5; //python:del
use crate::index::Pattern6; //python:del
use crate::index::Pattern7; //python:del
use crate::index::Pattern8; //python:del
use crate::index::INDEX_TABLE; //python:del
use crate::models::EDGE3; //python:del
use crate::models::EDGE4; //python:del
use crate::models::EDGE5; //python:del
use crate::models::EDGE6; //python:del
use crate::models::EDGE7; //python:del
use std::arch::x86_64::*; //python:del
use std::mem::transmute; //python:del

#[derive(Copy, Clone, Debug)]
pub struct Value([f32; 3]);
const ZERO_VALUE: Value = Value([0.0; 3]);

pub struct Connect {
    weight1: [[__m256; 15]; 8],
    bias1: __m256,
    weight2: [__m256; 8],
    bias2: __m256,
    weight3: __m256,
    bias3: f32,
}

const ZERO: __m256 = unsafe { transmute([0.0f32; 8]) };
impl Connect {
    fn load(data: &[f32]) -> Self {
        let mut offset = 0;
        let mut weight1 = [[0f32; 120]; 8];
        for i in 0..8 {
            for j in 0..(38 * 3) {
                weight1[i][j] = data[offset];
                offset += 1;
            }
        }
        let weight1: [[__m256; 15]; 8] = unsafe { transmute(weight1) };
        let mut bias1 = [0f32; 8];
        for i in 0..8 {
            bias1[i] = data[offset];
            offset += 1;
        }
        let bias1: __m256 = unsafe { transmute(bias1) };
        let mut weight2 = [[0f32; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                weight2[i][j] = data[offset];
                offset += 1;
            }
        }
        let weight2: [__m256; 8] = unsafe { transmute(weight2) };
        let mut bias2 = [0f32; 8];
        for i in 0..8 {
            bias2[i] = data[offset];
            offset += 1;
        }
        let bias2: __m256 = unsafe { transmute(bias2) };
        let mut weight3 = [0f32; 8];
        for i in 0..8 {
            weight3[i] = data[offset];
            offset += 1;
        }
        let weight3: __m256 = unsafe { transmute(weight3) };
        let bias3 = data[offset];
        eprintln!("offset: {}", offset);
        Self {
            weight1,
            bias1,
            weight2,
            bias2,
            weight3,
            bias3,
        }
    }

    #[target_feature(enable = "avx2,bmi2")]
    pub unsafe fn calc(&self, data: [__m256; 15]) -> f32 {
        let mut v = [0f32; 8];
        for i in 0..8 {
            let mut sum = ZERO;
            for j in 0..15 {
                sum = _mm256_fmadd_ps(data[j], self.weight1[i][j], sum);
            }
            v[i] = __m256_hori_sum(sum);
        }
        let mut v: __m256 = transmute(v);
        v = _mm256_add_ps(v, self.bias1);
        v = _mm256_max_ps(v, ZERO);
        let mut u = [0f32; 8];
        for i in 0..8 {
            u[i] = __m256_hori_sum(_mm256_mul_ps(v, self.weight2[i]));
        }
        let mut u: __m256 = transmute(u);
        u = _mm256_add_ps(u, self.bias2);
        u = _mm256_max_ps(u, ZERO);
        let res = __m256_hori_sum(_mm256_mul_ps(u, self.weight3));
        res + self.bias3
    }
}

pub struct Score {
    table3: Vec<Value>,
    table4: Vec<Value>,
    table5: Vec<Value>,
    table6: Vec<Value>,
    table7: Vec<Value>,
    table8: Vec<Value>,
    con: Connect,
}

impl Score {
    pub fn new(model8: &EDGE8, data: &[f32], con: &[f32]) -> Self {
        let mut table8 = Vec::with_capacity(Pattern8::len());
        let mut table7 = Vec::with_capacity(Pattern7::len());
        let mut table6 = Vec::with_capacity(Pattern6::len());
        let mut table5 = Vec::with_capacity(Pattern5::len());
        let mut table4 = Vec::with_capacity(Pattern4::len());
        let mut table3 = Vec::with_capacity(Pattern3::len());

        let mut p = Pattern8::new();
        let mut i = 0;
        let len = Pattern8::len();
        loop {
            let v = p.get();
            let v: T24 = unsafe { transmute(v) };
            let res = unsafe { model8.calc(v) };
            table8.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        let mut offset = 0;
        let e3 = EDGE3::load(&data[offset..]);
        let mut p = Pattern3::new();
        let mut i = 0;
        let len = Pattern3::len();
        loop {
            let v = p.get();
            let res = unsafe { e3.calc(&v) };
            table3.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        offset += EDGE3::size();
        let e4 = EDGE4::load(&data[offset..]);
        let mut p = Pattern4::new();
        let mut i = 0;
        let len = Pattern4::len();
        loop {
            let v = p.get();
            let res = unsafe { e4.calc(&v) };
            table4.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        offset += EDGE4::size();
        let e5 = EDGE5::load(&data[offset..]);
        let mut p = Pattern5::new();
        let mut i = 0;

        let len = Pattern5::len();
        loop {
            let v = p.get();
            let res = unsafe { e5.calc(&v) };
            table5.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        offset += EDGE5::size();
        let e6 = EDGE6::load(&data[offset..]);
        let mut p = Pattern6::new();
        let mut i = 0;
        let len = Pattern6::len();
        loop {
            let v = p.get();
            let res = unsafe { e6.calc(&v) };
            table6.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        offset += EDGE6::size();
        let e7 = EDGE7::load(&data[offset..]);
        let mut p = Pattern7::new();
        let mut i = 0;
        let len = Pattern7::len();
        loop {
            let v = p.get();
            let res = unsafe { e7.calc(&v) };
            table7.push(Value(res));
            i += 1;
            if i == len {
                break;
            }
            p.next();
        }

        let con = Connect::load(con);

        Self {
            table3,
            table4,
            table5,
            table6,
            table7,
            table8,
            con,
        }
    }

    #[inline]
    fn index(b: usize, w: usize, cb: usize, cw: usize) -> usize {
        unsafe {
            let mut res = *INDEX_TABLE[3].get_unchecked(b);
            res += *INDEX_TABLE[4].get_unchecked(w);
            res += *INDEX_TABLE[0].get_unchecked(cb);
            res += *INDEX_TABLE[1].get_unchecked(cw);
            res
        }
    }

    #[target_feature(enable = "avx2,bmi2")]
    pub unsafe fn score(&self, board: &Board) -> f32 {
        let lm = board.legal_moves();
        let lo = board.opp_legal_moves();
        let mut v = [ZERO_VALUE; 40];
        for i in 0..4 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table3.get_unchecked(index);
            *v.get_unchecked_mut(i) = t;
        }
        for i in 4..8 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table4.get_unchecked(index);
            *v.get_unchecked_mut(i) = t;
        }
        for i in 8..12 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table5.get_unchecked(index);
            *v.get_unchecked_mut(i) = t;
        }
        for i in 12..16 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table6.get_unchecked(index);
            *v.get_unchecked_mut(i) = t;
        }
        for i in 16..20 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table7.get_unchecked(index);
            *v.get_unchecked_mut(i) = t;
        }
        let x_me: [u8; 8] = transmute(board.me);
        let x_op: [u8; 8] = transmute(board.opp);
        let x_lm: [u8; 8] = transmute(lm);
        let x_lo: [u8; 8] = transmute(lo);
        for i in 0..8 {
            let index = Self::index(
                x_me[i] as usize,
                x_op[i] as usize,
                x_lm[i] as usize,
                x_lo[i] as usize,
            );
            let t = *self.table8.get_unchecked(index);
            *v.get_unchecked_mut(i + 20) = t;
        }
        for i in 20..30 {
            let mask = *MASKS.get_unchecked(i);
            let b = _pext_u64(board.me, mask);
            let w = _pext_u64(board.opp, mask);
            let cb = _pext_u64(lm, mask);
            let cw = _pext_u64(lo, mask);
            let index = Self::index(b as usize, w as usize, cb as usize, cw as usize);
            let t = *self.table8.get_unchecked(index);
            *v.get_unchecked_mut(i + 8) = t;
        }
        let v = transmute(v);
        self.con.calc(v)
    }
}
