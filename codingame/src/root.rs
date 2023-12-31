// python:replace use std::fmt;
// python:replace use std::io;
// python:replace use std::mem::transmute;
// python:replace use std::arch::x86_64::*;
// python:replace use std::ops;
//python:replace {data.rs}
//python:replace {bin.rs}
//python:replace {pos.rs}
//python:replace {base.rs}
//python:replace {read.rs}
//python:replace {book.rs}
//python:replace {mask.rs}
//python:replace {edge8.rs}
//python:replace {models.rs}
//python:replace {index.rs}
//python:replace {score.rs}
//python:replace {time.rs}
//python:replace {calc.rs}
//python:replace {ai.rs}

fn main() {
    let mut buf = String::new();
    let id = read_id(&mut buf);
    init_score();
    load_book();
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
