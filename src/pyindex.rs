use crate::base::Board;

impl Board {
    pub fn pyindex(&self, data: &[u8]) -> Vec<i32> {
        let len = data.len();
        let mut res = vec![0; len * 3];
        unsafe {
            let me = self.legal_moves();
            let opp = self.opp_legal_moves();
            for (i, p) in data.iter().enumerate() {
                let mask = 1 << p;
                let r = res.get_unchecked_mut(i);
                if self.me & mask != 0 {
                    *r = 1;
                } else if self.opp & mask != 0 {
                    *r = -1;
                }
                let r = res.get_unchecked_mut(i + len);
                if me & mask != 0 {
                    *r = 1;
                }
                let r = res.get_unchecked_mut(i + len * 2);
                if opp & mask != 0 {
                    *r = 1;
                }
            }
        }
        res
    }
}
