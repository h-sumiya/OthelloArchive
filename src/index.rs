macro_rules! pattern {
    ($name:ident,$len:expr) => {
        #[derive(Debug)]
        pub struct $name {
            now: [i32; $len],
            next: usize,
            val: i32,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    now: [0; $len],
                    next: 0,
                    val: 0,
                }
            }
            pub fn next(&mut self) {
                unsafe {
                    if self.val < 5 {
                        self.val += 1;

                        *self.now.get_unchecked_mut(self.next) = self.val;
                    } else {
                        self.next += 1;
                        for i in 0..self.next {
                            *self.now.get_unchecked_mut(i) = 0;
                        }
                        while self.next < ($len - 1) && *self.now.get_unchecked(self.next) == 5 {
                            *self.now.get_unchecked_mut(self.next) = 0;
                            self.next += 1;
                        }
                        *self.now.get_unchecked_mut(self.next) += 1;
                        self.next = 0;
                        self.val = 0;
                    }
                }
            }
            pub fn get(&self) -> [f32; $len * 3] {
                let mut res = [0.0; $len * 3];
                for i in 0..$len {
                    unsafe {
                        match self.now[i] {
                            0 => (),
                            1 => *res.get_unchecked_mut($len + i) = 1.0,
                            2 => *res.get_unchecked_mut($len * 2 + i) = 1.0,
                            3 => {
                                *res.get_unchecked_mut($len + i) = 1.0;
                                *res.get_unchecked_mut($len * 2 + i) = 1.0;
                            }
                            4 => *res.get_unchecked_mut(i) = 1.0,
                            5 => *res.get_unchecked_mut(i) = -1.0,
                            _ => panic!("not implemented"),
                        }
                    }
                }
                res
            }
            pub fn len() -> usize {
                6usize.pow($len as u32)
            }
        }
    };
}

pattern!(Pattern8, 8);
pattern!(Pattern7, 7);
pattern!(Pattern6, 6);
pattern!(Pattern5, 5);
pattern!(Pattern4, 4);
pattern!(Pattern3, 3);

const fn IndexTable() -> [[usize; 256]; 5] {
    let mut res = [[0; 256]; 5];
    let mut i = 0usize;
    let masks = [1 << 0, 1 << 1, 1 << 2, 1 << 3 , 1 << 4, 1 << 5, 1 << 6, 1 << 7];
    while i < 256 {
        i += 1;
        let mut j = 0usize;
        while j < 5 {
            let mut k = 0usize;
            while k < 8 {
                if i & masks[k] != 0 {
                    res[j][i] += 6usize.pow(k as u32) * (j + 1);
                }
                k += 1;
            }
            j += 1;
        }
    }
    res
}

pub const INDEX_TABLE: [[usize; 256]; 5] = IndexTable();
