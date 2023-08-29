mod ai;
mod base;
mod base2;
mod calc;
mod database;
mod mask;
mod pos;
mod read;
mod time;

use base::Board;
use base2::Boardv2;

fn main() {
    let mut test_data = String::new();
    std::io::stdin().read_line(&mut test_data).unwrap();
    let mut board = Board::new();
    let mut stream = vec![];
    for i in (0..test_data.len()).step_by(2) {
        let pos = pos::Pos::from_str(&test_data[i..i + 2]);
        board = board.put(pos);
        let board2 = Boardv2 {
            me: board.me,
            opp: board.opp,
        };
        stream.push((board.me, board.opp));
        //assert_eq!(board.legal_moves(), board2.legal_moves());
        println!("OK")
    }
    let mut sum = 0;
    let start = std::time::Instant::now();
    for (me, opp) in &stream {
        sum += Board { me: *me, opp: *opp }.legal_moves();
    }
    println!("v1:{:?} sum:{}", start.elapsed(), sum);
    let mut sum2 = 0;
    let start = std::time::Instant::now();
    for (me, opp) in &stream {
        sum2 += Boardv2 { me: *me, opp: *opp }.legal_moves();
    }
    println!("v2:{:?} sum:{}", start.elapsed(), sum2);
    assert_eq!(sum, sum2);
}
