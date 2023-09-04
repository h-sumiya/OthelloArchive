#[macro_use]
mod data;
mod ai_legacy;
mod ai;
mod base;
mod bin;
mod book;
mod calc;
mod calc_legacy;
mod disp;
mod mask;
mod pos;
mod read;
mod score;
mod time;

use base::Board;
use calc::set_default_score;

fn main() {
    let start = std::time::Instant::now();
    calc::first_load();
    calc::set_default_score(0);
    book::load_book();
    println!("load time: {}ms", start.elapsed().as_millis());
    let mut board = Board::new();
    let mut turn = false;
    let vs = false;
    loop {
        unsafe {
            if board.legal_moves() == 0 {
                board = board.pass();
                if board.legal_moves() == 0 {
                    break;
                }
                turn = !turn;
            }
        }
        let pos;
        if turn {
            pos = board.ai2();
            println!("ai: {}", pos);
        } else {
            println!("{}", board);
            if vs {
                loop {
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf).unwrap();
                    if let Ok(p) = pos::Pos::from_str(&buf) {
                        if p.0 & unsafe { board.legal_moves() } != 0 {
                            pos = p;
                            break;
                        }
                    }
                    println!("invalid input");
                }
            } else {
                pos = board.ai();
                println!("ai: {}", pos);
            }
        }
        board = unsafe { board.put(pos) };
        turn = !turn;
    }
    if !turn {
        board = board.pass();
    }
    println!("end\n{}", board);
    println!("score: {}", board.kn());
}
