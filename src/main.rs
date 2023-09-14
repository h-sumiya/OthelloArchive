#[macro_use]
mod data;
mod ai;
mod base;
mod bin;
mod book;
mod calc;
mod calc_legacy;
mod disp;
mod edge8;
mod index;
mod mask;
mod models;
mod pos;
mod read;
mod score;
mod time;

use std::mem::transmute;

use base::Board;

fn main() {
    let start = std::time::Instant::now();
    book::load_book();
    calc::init_score();
    println!("load time: {}ms", start.elapsed().as_millis());
    let mut board = Board::new();
    let (me,opp):(u64,u64) = unsafe {
        transmute::<[u8;16],_>([0, 0, 0, 64, 0, 0, 32, 16, 0, 0, 0, 56, 56, 248, 0, 0])
    };
    board.me = me;
    board.opp = opp;
    println!("{}", unsafe {board.defalut_score()});
    let mut turn = false;
    let vs = true;
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
            pos = board.ai();
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
