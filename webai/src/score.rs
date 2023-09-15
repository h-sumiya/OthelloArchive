use itertools::Itertools;
use once_cell::sync::Lazy;
use std::mem::transmute;
use std::ops;

macro_rules! linear_middle {
    ($name:ident, $input:expr , $output:expr) => {
        struct $name {
            weights: [[f32; $input]; $output],
            bias: [f32; $output],
        }

        impl $name {
            fn size() -> usize {
                $input * $output + $output
            }

            fn load(data: &[f32]) -> Self {
                let mut weights = [[0.0; $input]; $output];
                let mut bias = [0.0; $output];
                for i in 0..$output {
                    for j in 0..$input {
                        weights[i][j] = data[i * $input + j];
                    }
                }
                for i in 0..$output {
                    bias[i] = data[$input * $output + i];
                }
                Self { weights, bias }
            }

            fn calc(&self, input: &[f32; $input]) -> [f32; $output] {
                let mut output = [0.0; $output];
                for i in 0..$output {
                    for j in 0..$input {
                        output[i] += input[j] * self.weights[i][j];
                    }
                    output[i] += self.bias[i];
                    if output[i] < 0.0 {
                        output[i] = 0.0;
                    }
                }
                output
            }
        }
    };
}

macro_rules! linear_last {
    ($name:ident, $input:expr) => {
        struct $name {
            weights: [f32; $input],
            bias: f32,
        }

        impl $name {
            fn size() -> usize {
                $input + 1
            }
            fn load(data: &[f32]) -> Self {
                let mut weights = [0.0; $input];
                for i in 0..$input {
                    weights[i] = data[i];
                }
                let bias = data[$input];
                Self { weights, bias }
            }

            fn calc(&self, input: &[f32; $input]) -> f32 {
                let mut output = 0.0;
                for i in 0..$input {
                    output += input[i] * self.weights[i];
                }
                output += self.bias;
                output
            }
        }
    };
}

macro_rules! models {
    ($name:ident, $first:ty , $input:expr , $last:ty , $($names:ident , $middle:ty),*) => {
        struct $name {
            first: $first,
            $(
                $names: $middle,
            )*
            last: $last,
        }

        impl $name {
            fn size() -> usize {
                <$first>::size() + $(<$middle>::size())* + <$last>::size()
            }

            fn load(data: &[f32]) -> Self {
                let mut offset = 0;
                let first = <$first>::load(&data[offset..]);
                offset += <$first>::size();
                $(
                    let $names = <$middle>::load(&data[offset..]);
                    offset += <$middle>::size();
                )*
                let last = <$last>::load(&data[offset..]);
                Self { first, $($names),* , last }
            }

            fn calc(&self, input: &[f32; $input]) -> f32 {
                let mut output = self.first.calc(input);
                $(
                    output = self.$names.calc(&output);
                )*
                self.last.calc(&output)
            }
        }
    };
}

linear_middle!(Linear8x8, 8, 8);
linear_middle!(Linear10x10, 10, 10);
linear_last!(Linear8, 8);
linear_last!(Linear10, 10);
models!(Cross, Linear8x8, 8, Linear8, a, Linear8x8);
models!(Edge, Linear10x10, 10, Linear10, a, Linear10x10);
models!(Corner, Linear10x10, 10, Linear10, a, Linear10x10);

struct Model {
    edge: Edge,
    corner: Corner,
    cross: Cross,
    cn: (f32, f32),
}

impl Model {
    fn load(data: &[f32]) -> Self {
        let mut offset = 0;
        let edge = Edge::load(&data[offset..]);
        offset += Edge::size();
        let corner = Corner::load(&data[offset..]);
        offset += Corner::size();
        let cross = Cross::load(&data[offset..]);
        offset += Cross::size();
        let cn = (data[offset], data[offset + 1]);
        Self {
            edge,
            corner,
            cross,
            cn,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Score {
    edge: Vec<f32>,
    corner: Vec<f32>,
    cross: Vec<f32>,
    cn_w: f32,
    cn_b: f32,
}

impl Score {
    fn load(model: &Model) -> Self {
        unsafe {
            let pattern = vec![0.0f32, 1.0, -1.0];
            let mut edge = Vec::with_capacity(59049);
            let mut corner = Vec::with_capacity(59049);
            let mut cross = Vec::with_capacity(6561);
            edge.set_len(59049);
            corner.set_len(59049);
            cross.set_len(6561);
            for (index, input) in itertools::iproduct!(
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter()
            )
            .enumerate()
            {
                let input = [
                    *input.9, *input.8, *input.7, *input.6, *input.5, *input.4, *input.3, *input.2,
                    *input.1, *input.0,
                ];
                *edge.get_unchecked_mut(index) = model.edge.calc(&input);
                *corner.get_unchecked_mut(index) = model.corner.calc(&input);
            }
            for (index, input) in itertools::iproduct!(
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter(),
                pattern.iter()
            )
            .enumerate()
            {
                let input = [
                    *input.7, *input.6, *input.5, *input.4, *input.3, *input.2, *input.1, *input.0,
                ];
                *cross.get_unchecked_mut(index) = model.cross.calc(&input);
            }
            Self {
                edge,
                corner,
                cross,
                cn_w: model.cn.0,
                cn_b: model.cn.1,
            }
        }
    }

    pub fn calc(&self, index: Index, cn: isize) -> f32 {
        unsafe {
            let mut score = 0.0;
            let index = index.0;
            for i in 0..4 {
                score += self.edge.get_unchecked(index[i]);
            }
            for i in 4..8 {
                score += self.corner.get_unchecked(index[i]);
            }
            for i in 8..10 {
                score += self.cross.get_unchecked(index[i]);
            }
            score += cn as f32 * self.cn_w + self.cn_b;
            score
        }
    }
}

const DATA: [f32; 26531] = unsafe { transmute(*include_bytes!("ai.bin")) };
pub static SCORE_TABLE: Lazy<Vec<Score>> = Lazy::new(|| {
    let locs: [usize; 61] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    ];
    let mut scores: Vec<Score> = vec![];
    for i in 1..61 {
        let loc = locs[i] * 617;
        let model = Model::load(&DATA[loc..]);
        scores.push(Score::load(&model));
    }
    scores
});

#[derive(Clone, Copy, Debug)]
pub struct Index([usize; 10]);

impl ops::Add<Index> for Index {
    type Output = Index;
    fn add(self, rhs: Index) -> Self::Output {
        let mut output = [0; 10];
        for i in 0..10 {
            output[i] = self.0[i] + rhs.0[i];
        }
        Index(output)
    }
}

impl ops::AddAssign<Index> for Index {
    fn add_assign(&mut self, rhs: Index) {
        for i in 0..10 {
            self.0[i] += rhs.0[i];
        }
    }
}

impl ops::Mul<usize> for Index {
    type Output = Index;
    fn mul(self, rhs: usize) -> Self::Output {
        let mut output = [0; 10];
        for i in 0..10 {
            output[i] = self.0[i] * rhs;
        }
        Index(output)
    }
}

const INDEXES: [[[u16; 10]; 8]; 8] = unsafe { transmute(*include_bytes!("index.bin")) };

pub static INDEX_TABLE: Lazy<Vec<Vec<Index>>> = Lazy::new(|| {
    let bits = [1, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7];
    let mut table: Vec<Vec<Index>> = Vec::with_capacity(8);
    for i in 0..8 {
        let line_index = INDEXES[i];
        let mut line = Vec::with_capacity(256);
        for j in 0..256usize {
            let mut index = [0usize; 10];
            for k in 0..8 {
                if bits[k] & j != 0 {
                    for l in 0..10 {
                        index[l] += line_index[k][l] as usize;
                    }
                }
            }
            line.push(Index(index));
        }
        table.push(line)
    }
    table
});
