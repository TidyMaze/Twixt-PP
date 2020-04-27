use std::io;
use rand::prelude::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const GRID_SIZE: usize = 12;

const HORIZONTAL: u8 = 1;
const VERTICAL: u8 = 2;

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

type Peg = (u8, u8);

type Coord = (u8, u8);

type Segment = (Peg, Peg);

fn make_grid() -> Grid {
    [[0; GRID_SIZE]; GRID_SIZE]
}

fn parse_peg(raw: String) -> Peg {
    let split: Vec<char> = raw.chars().collect();
    let res = (((split[0] as u32) - ('A' as u32)) as u8, split.iter().skip(1).collect::<String>().parse::<u8>().unwrap() - 1);
    eprintln!("parsed {} as {:?}", raw, res);
    res
}

fn show_grid(g: Grid) -> String {
    return g.iter().map(|l| l.iter().map(|v| v.to_string() + " ").collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
}

fn int_to_alpha(v: u8) -> char {
    ((v as u32) + ('A' as u32)) as u8 as char
}

fn main() {
    let mut rng = thread_rng();

    let mut my_lines = 0;

    let mut turn = 0;
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let his_last_peg = input_line.trim().to_string();

        if turn == 0 {
            if his_last_peg == "FIRST" {
                my_lines = HORIZONTAL;
            } else if his_last_peg == "SWAP" {
                my_lines = HORIZONTAL;
            } else {
                my_lines = VERTICAL;
                turn += 1;
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_your_pegs = parse_input!(input_line, i32); // The number of pegs you have on the board.

        let mut my_segments: Vec<Segment> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) / 2);
        let mut opp_segments: Vec<Segment> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) / 2);

        let mut grid = make_grid();

        for _ in 0..num_your_pegs as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            eprintln!("in {}", input_line);
            let p = parse_peg(input_line.trim().to_string());
            grid[p.1 as usize][p.0 as usize] = 1;
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_your_segments = parse_input!(input_line, i32); // The number of segments you have on the board.
        for _ in 0..num_your_segments as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let your_seg_peg_1 = parse_peg(inputs[0].trim().to_string()); // The first end of one of your segments.
            let your_seg_peg_2 = parse_peg(inputs[1].trim().to_string()); // The second end of one of your segments.

            my_segments.push((your_seg_peg_1, your_seg_peg_2));
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_his_pegs = parse_input!(input_line, i32); // The number of pegs your opponent has on the board.
        for _ in 0..num_his_pegs as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let p = parse_peg(input_line.trim().to_string());
            grid[p.1 as usize][p.0 as usize] = 2;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_his_segments = parse_input!(input_line, i32); // The number of segments of your opponent.
        for _ in 0..num_his_segments as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let his_seg_peg_1 = parse_peg(inputs[0].trim().to_string()); // The first end of one of his segments.
            let his_seg_peg_2 = parse_peg(inputs[1].trim().to_string()); // The second end of one of his segments.
            opp_segments.push((his_seg_peg_1, his_seg_peg_2));
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        eprintln!("grid is \n{}", show_grid(grid));
        eprintln!("my segments are\n{:?}", my_segments);
        eprintln!("opp segments are\n{:?}", opp_segments);

        let a = random_pick(&mut rng, &grid, my_lines);

        eprintln!("{}{}", a.0, a.1);
        println!("{}{}", int_to_alpha(a.0), a.1 + 1);
        turn +=1;
    }

    fn random_pick(rng: &mut ThreadRng, g: &Grid, my_lines: u8) -> Peg {
        loop {
            let x: u8 = rng.gen_range(0, GRID_SIZE as u8);
            let y: u8 = rng.gen_range(0, GRID_SIZE as u8);
            if g[y as usize][x as usize] == 0 {
                if my_lines == HORIZONTAL && (x == 0 || x == (GRID_SIZE - 1) as u8) {
                    continue;
                }
                if my_lines == VERTICAL && (y == 0 || y == (GRID_SIZE - 1) as u8) {
                    continue;
                }
                return (x as u8, y as u8);
            }
        }
    }

    fn play_action(mut g: Grid, c: Coord, p: u8) -> Grid {
        g[c.1 as usize][c.0 as usize] = p;
        g
    }
}
