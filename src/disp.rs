use crate::base::Board;
use crate::mask::POSES;
use crate::pos::{Pos, LABEL_Y};
use crate::score::Index;
use std::fmt;

impl fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indexes = self.indexes();
        write!(f, "[")?;
        for i in 0..10 {
            write!(f, "{},", indexes[i])?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=========================")?;
        write!(f, "legal_moves: ")?;
        let legal_moves = unsafe { self.legal_moves() };
        for pos in POSES {
            if legal_moves & pos != 0 {
                write!(f, "{} ", Pos(pos))?;
            }
        }
        writeln!(f)?;
        writeln!(f, "  a b c d e f g h")?;
        let mut me = self.me;
        let mut opp = self.opp;
        for y in LABEL_Y.iter() {
            write!(f, "{} ", y)?;
            for _ in 0..8 {
                if me & 1 != 0 {
                    write!(f, "○ ")?;
                } else if opp & 1 != 0 {
                    write!(f, "● ")?;
                } else {
                    write!(f, ". ")?;
                }
                me >>= 1;
                opp >>= 1;
            }
            writeln!(f)?;
        }
        writeln!(f, "=========================")?;
        Ok(())
    }
}
