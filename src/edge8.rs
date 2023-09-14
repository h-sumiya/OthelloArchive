use std::arch::x86_64::*; //python:del
use std::mem::transmute; //python:del

#[derive(Copy, Clone)]
pub struct T24(__m256, __m256, __m256);

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn __m256_hori_sum(a: __m256) -> f32 {
    let low = _mm256_castps256_ps128(a);
    let high = _mm256_extractf128_ps(a, 1);
    let sum = _mm_add_ps(low, high);
    let sum = _mm_hadd_ps(sum, sum);
    let sum = _mm_hadd_ps(sum, sum);
    _mm_cvtss_f32(sum)
}

impl T24 {
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn mul(&self, b: T24) -> T24 {
        let a0 = _mm256_mul_ps(self.0, b.0);
        let a1 = _mm256_mul_ps(self.1, b.1);
        let a2 = _mm256_mul_ps(self.2, b.2);
        T24(a0, a1, a2)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn add(&self, b: T24) -> T24 {
        let a0 = _mm256_add_ps(self.0, b.0);
        let a1 = _mm256_add_ps(self.1, b.1);
        let a2 = _mm256_add_ps(self.2, b.2);
        T24(a0, a1, a2)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn relu(&self) -> T24 {
        let zero = _mm256_setzero_ps();
        let a0 = _mm256_max_ps(self.0, zero);
        let a1 = _mm256_max_ps(self.1, zero);
        let a2 = _mm256_max_ps(self.2, zero);
        T24(a0, a1, a2)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn sum(&self) -> f32 {
        let temp: __m256 = _mm256_add_ps(self.0, self.1);
        let temp: __m256 = _mm256_add_ps(temp, self.2);
        __m256_hori_sum(temp)
    }
}

#[derive(Copy, Clone)]
pub struct T16(__m256, __m256);

impl T16 {
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn mul(&self, b: T16) -> T16 {
        let a0 = _mm256_mul_ps(self.0, b.0);
        let a1 = _mm256_mul_ps(self.1, b.1);
        T16(a0, a1)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn add(&self, b: T16) -> T16 {
        let a0 = _mm256_add_ps(self.0, b.0);
        let a1 = _mm256_add_ps(self.1, b.1);
        T16(a0, a1)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn relu(&self) -> T16 {
        let zero = _mm256_setzero_ps();
        let a0 = _mm256_max_ps(self.0, zero);
        let a1 = _mm256_max_ps(self.1, zero);
        T16(a0, a1)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn sum(&self) -> f32 {
        let temp: __m256 = _mm256_add_ps(self.0, self.1);
        __m256_hori_sum(temp)
    }
}
pub struct EDGE8 {
    pub weights1: [T24; 24],
    pub bias1: T24,
    pub weights2: [T24; 16],
    pub bias2: T16,
    pub weights3: [T16; 16],
    pub bias3: T16,
    pub wieghts4: [T16; 3],
    pub bias4: [f32; 3],
}

impl EDGE8 {
    #[target_feature(enable = "avx2")]
    pub unsafe fn calc(&self, input: T24) -> [f32; 3] {
        let mut output = [0f32; 24];
        for i in 0..24 {
            output[i] = input.mul(self.weights1[i]).sum();
        }
        let output: T24 = transmute(output);
        let input = output.add(self.bias1).relu();
        let mut output = [0f32; 16];
        for i in 0..16 {
            output[i] = input.mul(self.weights2[i]).sum();
        }
        let output: T16 = transmute(output);
        let input = output.add(self.bias2).relu();
        let mut output = [0f32; 16];
        for i in 0..16 {
            output[i] = input.mul(self.weights3[i]).sum();
        }
        let output: T16 = transmute(output);
        let input = output.add(self.bias3).relu();
        let mut output = [0f32; 3];
        for i in 0..3 {
            output[i] = input.mul(self.wieghts4[i]).sum() + self.bias4[i];
        }
        output
    }
}