// python:replace use std::fmt;
// python:replace use std::io;
//python:replace {base.rs}
//python:replace {pos.rs}
//python:replace {read.rs}
//python:replace {mask.rs}
//python:replace {database.rs}
//python:replace {time.rs}
//python:replace {calc.rs}
//python:replace {ai.rs}

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