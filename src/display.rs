use crate::base::Board;
use crate::mask::POSES;
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut me = self.me;
        let mut opp = self.opp;
        for _ in 0..8 {
            for _ in 0..8 {
                if me & 1 != 0 {
                    write!(f, "○")?;
                } else if opp & 1 != 0 {
                    write!(f, "●")?;
                } else {
                    write!(f, ".")?;
                }
                me >>= 1;
                opp >>= 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}