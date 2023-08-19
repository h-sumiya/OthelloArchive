use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let id = parse_input!(input_line, i32); // id of your player.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let board_size = parse_input!(input_line, i32);

    // game loop
    loop {
        for i in 0..board_size as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim().to_string(); // rows from top to bottom (viewer perspective).
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // number of legal actions for this turn.
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let action = input_line.trim().to_string(); // the action
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("f4"); // a-h1-8
    }
}
