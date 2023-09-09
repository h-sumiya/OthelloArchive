use crate::base;
use std::mem::transmute;

impl base::Board {
    pub fn to_pydata(&self) -> [u8; 64] {
        let mut res = [0u8; 64];
        for i in 0..64 {
            let mask = 1 << i;
            if self.me & mask != 0 {
                res[i] = 1;
            } else if self.opp & mask != 0 {
                res[i] = 2;
            }
        }
        res
    }

    pub fn get(&self, p: usize) -> u8 {
        let mask = 1 << p;
        if self.me & mask != 0 {
            1
        } else if self.opp & mask != 0 {
            2
        } else {
            0
        }
    }

    pub fn pysave(&self) -> [u8; 16] {
        let me = self.me.to_le_bytes();
        let opp = self.opp.to_le_bytes();
        unsafe { std::mem::transmute([me, opp]) }
    }

    pub fn from_pydata(data: &[u8]) -> Self {
        let mut me = 0u64;
        let mut opp = 0u64;
        for i in 0..64 {
            let mask = 1 << i;
            if data[i] == 1 {
                me |= mask;
            } else if data[i] == 2 {
                opp |= mask;
            }
        }
        Self { me, opp }
    }

    pub fn pyload(data: &[u8]) -> Self {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(data);
        let [me, opp]: [u64; 2] = unsafe { transmute(buf) };
        Self { me, opp }
    }
}

pub fn u64_to_pydata(data: u64) -> Vec<u8> {
    let mut res = vec![];
    for i in 0..64u8 {
        let mask = 1 << i;
        if data & mask != 0 {
            res.push(i)
        }
    }
    res
}
