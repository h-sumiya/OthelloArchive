use crate::base::Board;
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..64 {
            let mask = 1 << i;
            if self.me & mask != 0 {
                write!(f, "1")?;
            } else if self.opp & mask != 0 {
                write!(f, "2")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}
