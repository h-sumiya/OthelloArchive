#[macro_use]
mod data;
mod ai;
mod base;
mod calc;
mod calc2;
mod display;
mod mask;
mod pos;
mod read;
mod score;
mod time;

use base::Board;

fn main() {
    score::temp();
}
fn sub() {
    let test_data = "e6f4g3d6c5c6c4f6d7e8b6e3f3c7d3b5a6b3f5b4f7g5g4c3c8d8f8e7b8h4h6h5h3g6h7a5b7g7c2d1c1e2e1d2f2f1g2b1a4a3a2h2h1g1a1b2g8h8a8a7";
    let mut board = Board::new();
    let mut stream = vec![];
    for i in (0..test_data.len()).step_by(2) {
        let pos = pos::Pos::from_str(&test_data[i..i + 2]);
        unsafe {
            board = board.put(pos);
        }
        stream.push((board.me, board.opp));
        if i % 4 == 0 {
            println!("{}", board);
        } else {
            println!("{}", board.pass());
        }
    }
}
