use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const GRID_SIZE: usize = 12;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

fn make_grid() -> Grid {
    [[0; GRID_SIZE]; GRID_SIZE]
}

fn parse_peg(raw: String) -> (u8, u8) {
    let split: Vec<char> = raw.chars().collect();
    eprintln!("split {:?}", split);
    (split[0] as u8 - ('A' as u8), split[1].to_string().parse::<u8>().unwrap())
}

fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let his_last_peg = input_line.trim().to_string(); // Your opponent's last peg (B3, or H10 for example), or FIRST, or SWAP.
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_your_pegs = parse_input!(input_line, i32); // The number of pegs you have on the board.

        let my_segments: Vec<(u8, u8)> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) / 2);
        let opp_segments: Vec<(u8, u8)> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) / 2);

        let mut grid = make_grid();

        for i in 0..num_your_pegs as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            eprintln!("in {}", input_line);
            let p = parse_peg(input_line.trim().to_string());
            grid[p.1 as usize][p.0 as usize] = 1;
        }

        eprintln!("grid is {:?}", grid);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_your_segments = parse_input!(input_line, i32); // The number of segments you have on the board.
        for i in 0..num_your_segments as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let your_seg_peg_1 = parse_peg(inputs[0].trim().to_string()); // The first end of one of your segments.
            let your_seg_peg_2 = parse_peg(inputs[1].trim().to_string()); // The second end of one of your segments.
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_his_pegs = parse_input!(input_line, i32); // The number of pegs your opponent has on the board.
        for i in 0..num_his_pegs as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let p = parse_peg(input_line.trim().to_string());
            grid[p.1 as usize][p.0 as usize] = 2;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_his_segments = parse_input!(input_line, i32); // The number of segments of your opponent.
        for i in 0..num_his_segments as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let his_seg_peg_1 = parse_peg(inputs[0].trim().to_string()); // The first end of one of his segments.
            let his_seg_peg_2 = parse_peg(inputs[1].trim().to_string()); // The second end of one of his segments.
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("G6 bla bla bla..."); // <Your new peg> <Optional commentary>
    }
}
