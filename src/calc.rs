use crate::base;

impl base::Board {
    pub fn count(&self) -> u32 {
        self.me.count_ones() + self.opp.count_ones()
    }

    pub fn counts(&self) -> (u32, u32) {
        (self.me.count_ones(), self.opp.count_ones())
    }

    pub fn cns(&self) -> (u32, u32) {
        unsafe {
            let me = self.legal_moves().count_ones();
            let opp = self.opp_legal_moves().count_ones();
            (me, opp)
        }
    }
}
