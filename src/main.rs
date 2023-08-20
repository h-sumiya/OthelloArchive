// python:replace use std::fmt;
// python:replace use std::io;
mod base; //python:replace {base.rs}
mod pos; //python:replace {pos.rs}
mod read; //python:replace {read.rs}
mod mask; //python:replace {mask.rs}
mod database; //python:replace {database.rs}
mod time; //python:replace {time.rs}
mod calc; //python:replace {calc.rs}
mod ai; //python:replace {ai.rs}

use base::Board; //python:del
use read::{read_id, read_line, skip_after}; //python:del

fn main() {
    let mut buf = String::new();
    let id = read_id(&mut buf);
    let mut board = Board::new();
    loop {
        for _ in 0..8 {
            read_line(&mut buf);
            board.push(&buf, id);
        }
        skip_after(&mut buf);
        let pos = board.ai();
        println!("{}", pos);
    }
}
