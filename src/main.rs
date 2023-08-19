fn main() {
    println!("Hello, world!");
}

enum Player {
    Black,
    White,
}

struct Board {
    player: u64,
    opponent: u64,
    turn: Player,
    progress: usize,
}

impl Board {
    fn new() -> Board {
        Board {
            player: 0x0000000810000000,
            opponent: 0x0000001008000000,
            turn: Player::Black,
            progress: 4,
        }
    }
}
