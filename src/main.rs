mod base; 
mod pos; 
mod read; 
mod mask; 
mod database; 
mod time; 
mod calc; 
mod ai; 

use base::Board; 
use read::{read_id, read_line, skip_after};

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
