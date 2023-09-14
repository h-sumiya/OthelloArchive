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

            #[target_feature(enable = "avx2")]
            unsafe fn calc(&self, input: &[f32; $input]) -> [f32; $output] {
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
            weights: [[f32; $input]; 3],
            bias: [f32; 3],
        }

        impl $name {
            fn size() -> usize {
                $input * 3 + 3
            }

            fn load(data: &[f32]) -> Self {
                let mut weights = [[0.0; $input]; 3];
                let mut bias = [0.0; 3];
                for i in 0..3 {
                    for j in 0..$input {
                        weights[i][j] = data[i * $input + j];
                    }
                }
                for i in 0..3 {
                    bias[i] = data[$input * 3 + i];
                }
                Self { weights, bias }
            }

            #[target_feature(enable = "avx2")]
            unsafe fn calc(&self, input: &[f32; $input]) -> [f32; 3] {
                let mut output = [0.0; 3];
                for i in 0..3 {
                    for j in 0..$input {
                        output[i] += input[j] * self.weights[i][j];
                    }
                    output[i] += self.bias[i];
                }
                output
            }
        }
    };
}

macro_rules! models {
    ($name:ident, $first:ty , $input:expr , $last:ty , $($names:ident , $middle:ty),*) => {
        pub struct $name {
            first: $first,
            $(
                $names: $middle,
            )*
            last: $last,
        }

        impl $name {
            pub fn size() -> usize {
                <$first>::size() + $(<$middle>::size() + )* <$last>::size()
            }

            pub fn load(data: &[f32]) -> Self {
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

            #[target_feature(enable = "avx2")]
            pub unsafe fn calc(&self, input: &[f32; $input]) -> [f32;3] {
                let output = self.first.calc(input);
                $(
                    let output = self.$names.calc(&output);
                )*
                self.last.calc(&output)
            }
        }
    };
}

linear_middle!(Linear21_21, 21, 21);
linear_middle!(Linear21_14, 21, 14);
linear_middle!(Linear14_14, 14, 14);
linear_last!(Linear14_3, 14);

linear_middle!(Linear18_18, 18, 18);
linear_middle!(Linear18_12, 18, 12);
linear_middle!(Linear12_12, 12, 12);
linear_last!(Linear12_3, 12);

linear_middle!(Linear15_15, 15, 15);
linear_middle!(Linear15_10, 15, 10);
linear_middle!(Linear10_10, 10, 10);
linear_last!(Linear10_3, 10);

//linear_middle!(Linear12_12, 12, 12);
linear_middle!(Linear12_8, 12, 8);
linear_middle!(Linear8_8, 8, 8);
linear_last!(Linear8_3, 8);

linear_middle!(Linear9_9, 9, 9);
linear_middle!(Linear9_6, 9, 6);
linear_middle!(Linear6_6, 6, 6);
linear_last!(Linear6_3, 6);

models!(
    EDGE7,
    Linear21_21,
    21,
    Linear14_3,
    a,
    Linear21_14,
    b,
    Linear14_14
);
models!(
    EDGE6,
    Linear18_18,
    18,
    Linear12_3,
    a,
    Linear18_12,
    b,
    Linear12_12
);
models!(
    EDGE5,
    Linear15_15,
    15,
    Linear10_3,
    a,
    Linear15_10,
    b,
    Linear10_10
);
models!(
    EDGE4,
    Linear12_12,
    12,
    Linear8_3,
    a,
    Linear12_8,
    b,
    Linear8_8
);
models!(EDGE3, Linear9_9, 9, Linear6_3, a, Linear9_6, b, Linear6_6);
