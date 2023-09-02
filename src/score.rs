use std::arch::x86_64::*; //python:del
use std::mem::transmute; //python:del
use std::ops; //python:del

const fn _pts(p: u32) -> f32 {
    match p {
        0 => 0f32,
        1 => 1f32,
        2 => -1f32,
        _ => 0f32,
    }
}

const fn pattern_table(p: [u32; 10]) -> T10 {
    let mut i = 0;
    let mut buf = [0f32; 8];
    while i < 8 {
        buf[i] = _pts(p[i]);
        i += 1;
    }
    T10 {
        0: T8::new([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ]),
        1: _pts(p[8]),
        2: _pts(p[9]),
    }
}

const fn pattern() -> [T10; 59049] {
    let mut table: [T10; 59049] = unsafe { transmute([[0f32; 16]; 59049]) };
    let mut now = [0u32; 10];
    let mut next = 0;
    let mut val = 0;
    let mut i = 0;
    while i < 59049 {
        table[i] = pattern_table(now);
        if val < 2 {
            val += 1;
            now[next] = val;
        } else {
            next += 1;
            let mut j = 0;
            while j < next {
                now[j] = 0;
                j += 1;
            }
            while next < 9 && now[next] == 2 {
                now[next] = 0;
                next += 1;
            }
            now[next] += 1;
            next = 0;
            val = 0;
        }
        i += 1;
    }
    table
}

const PATTERN_TABLE: [T10; 59049] = pattern();

const RELU: __m256 = unsafe { transmute([0f32; 8]) };

#[derive(Clone, Copy, Debug)]
struct T8(__m256);
#[derive(Clone, Copy, Debug)]
struct T10(T8, f32, f32);

impl T8 {
    const fn new(data: [f32; 8]) -> Self {
        unsafe { T8(transmute(data)) }
    }
    fn sum(&self) -> f32 {
        unsafe {
            let v: [f32; 8] = transmute(self.0);
            let mut sum = 0f32;
            for i in 0..8 {
                sum += v.get_unchecked(i);
            }
            sum
        }
    }
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn relu(&self) -> Self {
        unsafe { T8(_mm256_max_ps(self.0, RELU)) }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn _add(a: __m256, b: __m256) -> __m256 {
    unsafe { _mm256_add_ps(a, b) }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn _mul(a: __m256, b: __m256) -> __m256 {
    unsafe { _mm256_mul_ps(a, b) }
}

impl ops::Add<T8> for T8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { T8(_add(self.0, rhs.0)) }
    }
}

impl ops::Mul<T8> for T8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        unsafe { T8(_mul(self.0, rhs.0)) }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn _mul_f32(a: __m256, b: f32) -> __m256 {
    unsafe { _mm256_mul_ps(a, _mm256_set1_ps(b)) }
}

impl ops::Mul<f32> for T8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        unsafe { T8(_mul_f32(self.0, rhs)) }
    }
}

struct Linear10 {
    w: [T8; 10],
    w1: [T8; 2],
    w2: [f32; 4],
    b: T8,
    b1: f32,
    b2: f32,
}

impl Linear10 {
    fn load(data: &[f32]) -> Self {
        let mut w = [T8::new([0f32; 8]); 10];
        let mut w1 = [[0f32; 8]; 2];
        let mut offset = 0;
        for i in 0..10 {
            w[i] = unsafe {
                let mut buf = [0f32; 8];
                buf.copy_from_slice(&data[offset..offset + 8]);
                transmute(buf)
            };
            offset += 10;
        }
        offset = 0;
        for i in 0..8 {
            w1[0][i] = data[offset + 8];
            w1[1][i] = data[offset + 9];
            offset += 10;
        }
        let w2 = [data[88], data[89], data[98], data[99]];
        let b: T8 = unsafe {
            let mut buf = [0f32; 8];
            buf.copy_from_slice(&data[100..108]);
            transmute(buf)
        };
        let b1 = data[108];
        let b2 = data[109];
        Self {
            w,
            w1: unsafe { transmute(w1) },
            w2,
            b,
            b1,
            b2,
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn calc(&self, input: &T10) -> T10 {
        unsafe {
            let mut output = [0f32; 8];
            for i in 0..8 {
                *output.get_unchecked_mut(i) = (*self.w.get_unchecked(i) * input.0).sum();
            }
            let output = (transmute::<_, T8>(output)
                + *self.w1.get_unchecked(0) * input.1
                + *self.w1.get_unchecked(1) * input.2
                + self.b)
                .relu();
            let mut out1 = (*self.w.get_unchecked(8) * input.0).sum()
                + *self.w2.get_unchecked(0) * input.1
                + *self.w2.get_unchecked(1) * input.2
                + self.b1;
            if out1 < 0f32 {
                out1 = 0f32;
            }
            let mut out2 = (*self.w.get_unchecked(9) * input.0).sum()
                + *self.w2.get_unchecked(2) * input.1
                + *self.w2.get_unchecked(3) * input.2
                + self.b2;
            if out2 < 0f32 {
                out2 = 0f32;
            }
            T10(output, out1, out2)
        }
    }
}

struct Model10 {
    l1: Linear10,
    l2: Linear10,
    w: (T8, f32, f32),
    b: f32,
}

impl Model10 {
    fn load(data: &[f32]) -> Self {
        Self {
            l1: Linear10::load(&data[0..110]),
            l2: Linear10::load(&data[110..220]),
            w: unsafe {
                let mut buf = [0f32; 8];
                buf.copy_from_slice(&data[220..228]);
                (transmute(buf), data[228], data[229])
            },
            b: data[230],
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn calc(&self, input: &T10) -> f32 {
        unsafe {
            let output = self.l1.calc(input);
            let output = self.l2.calc(&output);
            (output.0 * self.w.0).sum() + (output.1 * self.w.1) + (output.2 * self.w.2) + self.b
        }
    }
}

struct Linear8 {
    w: [T8; 8],
    b: T8,
}

impl Linear8 {
    fn new(data: &[f32]) -> Self {
        let mut buf: [f32; 64] = [0f32; 64];
        buf.copy_from_slice(&data[0..64]);
        let w = unsafe { transmute(buf) };
        let mut buf: [f32; 8] = [0f32; 8];
        buf.copy_from_slice(&data[64..72]);
        let b = unsafe { transmute(buf) };
        Self { w, b }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn calc(&self, data: &T8) -> T8 {
        let mut output = [0f32; 8];
        unsafe {
            for i in 0..8 {
                *output.get_unchecked_mut(i) = (*self.w.get_unchecked(i) * *data).sum();
            }
            (transmute::<_, T8>(output) + self.b).relu()
        }
    }
}

struct Model8 {
    l1: Linear8,
    l2: Linear8,
    w: T8,
    b: f32,
}

impl Model8 {
    fn load(data: &[f32]) -> Self {
        Self {
            l1: Linear8::new(&data[0..72]),
            l2: Linear8::new(&data[72..144]),
            w: unsafe {
                let mut buf = [0f32; 8];
                buf.copy_from_slice(&data[144..152]);
                transmute(buf)
            },
            b: data[152],
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn calc(&self, data: &T8) -> f32 {
        let output = self.l1.calc(data);
        let output = self.l2.calc(&output);
        (output * self.w).sum() + self.b
    }
}

pub struct Score {
    edge: Vec<f32>,
    corner: Vec<f32>,
    cross: Vec<f32>,
    cn_w: f32,
    cn_b: f32,
}

impl Score {
    #[target_feature(enable = "avx2")]
    pub unsafe fn load(data: &[f32]) -> Self {
        let edge_model = Model10::load(&data);
        let corner_model = Model10::load(&data[231..]);
        let cross_model = Model8::load(&data[462..]);
        unsafe {
            let mut edge = Vec::with_capacity(59049);
            let mut corner = Vec::with_capacity(59049);
            let mut cross = Vec::with_capacity(6561);
            edge.set_len(59049);
            corner.set_len(59049);
            cross.set_len(6561);
            for i in 0..59049 {
                *edge.get_unchecked_mut(i) = edge_model.calc(&PATTERN_TABLE[i]);
            }
            for i in 0..59049 {
                *corner.get_unchecked_mut(i) = corner_model.calc(&PATTERN_TABLE[i]);
            }
            for i in 0..6561 {
                *cross.get_unchecked_mut(i) = cross_model.calc(&PATTERN_TABLE[i].0);
            }
            Self {
                edge,
                corner,
                cross,
                cn_w: data[615],
                cn_b: data[616],
            }
        }
    }
    pub unsafe fn calc(&self, index: Index, cn: isize) -> f32 {
        let indexes = index.indexes();
        let mut score = 0f32;
        for i in 0..4 {
            score += self.edge.get_unchecked(indexes[i]);
        }
        for i in 4..8 {
            score += self.corner.get_unchecked(indexes[i]);
        }
        for i in 8..10 {
            score += self.cross.get_unchecked(indexes[i]);
        }
        score += cn as f32 * self.cn_w + self.cn_b;
        score
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Index(__m256i);
impl Index {
    const fn from_index(data: &[u16; 10]) -> Self {
        unsafe {
            transmute([
                data[8], data[0], data[1], data[2], data[9], data[3], data[4], data[5], 0, data[6],
                data[7], 0, 0, 0, 0, 0,
            ])
        }
    }
    pub fn indexes(&self) -> [usize; 10] {
        unsafe {
            let v: [u16; 16] = transmute(self.0);
            let mut buf = [0usize; 10];
            for (i, p) in [1, 2, 3, 5, 6, 7, 9, 10, 0, 4].iter().enumerate() {
                *buf.get_unchecked_mut(i) = *v.get_unchecked(*p) as usize;
            }
            buf
        }
    }
    const fn add(mut d1: [u16; 10], d2: &[u16; 10]) -> [u16; 10] {
        let mut i = 0;
        while i < 10 {
            d1[i] += d2[i];
            i += 1;
        }
        d1
    }
    #[inline]
    #[target_feature(enable = "avx2")]
    pub unsafe fn x2(&self) -> Self {
        unsafe { Index(_mm256_slli_epi16(self.0, 1)) }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn add_index(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_add_epi16(a, b) }
}

impl ops::Add<Index> for Index {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { Index(add_index(self.0, rhs.0)) }
    }
}

impl ops::AddAssign<Index> for Index {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = add_index(self.0, rhs.0);
        }
    }
}

const INDEXES:[[[u16;10];8];8] = include_data!(1280, "、　　ン、　　　、　　　　　〃　　　　　〃　　　〉　　　　　〉　　　〛　　　　　〛　　　　糣　　　　け　　　　㢋　　　　　　　　　け　　　　ン、　　　、　　　㢋　　　　け　　　　　　　　　ン　　　〃　㋙　　　㋙　　　　　㢋　　　　　　　　　䦡　　　　　　　　　糣　　　　䦡　　　　　　　　　ン　　　㋙　　　　　〃　　　　　　　け㢋　　　　　　　　糣䦡　　　　　　　　　　　　　〉　　　　　　　　　　　　　　　　　　　　　　　　　　　　　　ン　㋙　　　㋙　　　　　〃　　　〉　　　　　　　〛糣　　　　　　　　䦡　　　　　　　　　　　　　　　　　　　　　　　　〛　　　　　　　　　　け　　　　　　　　　　　㢋　　　　　　　　　〉　　　〛　　　　　　　〉　　　〛　　　　　㢋　　　　　　　　　　　　　　　　　　　　　　　　　〛　　　　　　　　け　　　　　　　　　　　　䦡　　　　　　　　　〛　　　　糣　　　　　　〃　　　〉　　　　　㋙　　　㋙　　　　　　　　　　　〉　　　　　　　　　　　　　　　　　　　　　　　　　　　　ン　　糣　　　　䦡　　　　け　　　　㢋　　　　　　　　　　〃　　　　　　　　　ン　〃　　糣　　　　䦡　　　　䦡　　　　　　　　　㢋　　　　　　　　　㋙　　　㋙　　　　　　　　　ン　㋙　　　　　　　け　　　　　ン、　　　、　、　　　　　　　け　　　　け　　　　㢋　　　　〛　　　　糣　　　　〉　　　〛　　　　　〃　　　〉　　　　　　　　　〃　　　　ン、　　　、　㢋　");
const fn index_table() -> [[Index; 256]; 8] {
    let bits = [1, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7];
    unsafe {
        let mut table: [[Index; 256]; 8] = transmute([0u8; 65536]);
        let mut i = 0;
        while i < 8 {
            let mut j = 0;
            let line_index = INDEXES[i];
            let mut line: [Index; 256] = transmute([0u8; 8192]);
            while j < 256 {
                let mut index = [0u16; 10];
                let mut k = 0;
                while k < 8 {
                    if bits[k] & j != 0 {
                        index = Index::add(index, &line_index[k]);
                    }
                    k += 1;
                }
                line[j] = Index::from_index(&index);
                j += 1;
            }
            table[i] = line;
            i += 1;
        }
        table
    }
}
pub const INDEX_TABLE: [[Index; 256]; 8] = index_table();
