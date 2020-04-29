use std::io;
use rand::prelude::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const GRID_SIZE: usize = 12;

const HORIZONTAL: u8 = 1;
const VERTICAL: u8 = 2;

type Grid = [[(u8, [bool; 8]); GRID_SIZE]; GRID_SIZE];

#[derive(Debug)]
struct Coord {
    x: u8,
    y: u8,
}

fn make_grid() -> Grid {
    [[(0, [false; 8]); GRID_SIZE]; GRID_SIZE]
}

fn parse_peg(raw: String) -> Coord {
    let split: Vec<char> = raw.chars().collect();
    Coord {
        x: ((split[0] as u32) - ('A' as u32)) as u8,
        y: split.iter().skip(1).collect::<String>().parse::<u8>().unwrap() - 1,
    }
}

fn show_grid(g: &Grid) -> String {
    return g.iter().map(|l| l.iter().map(|(v, _)| v.to_string() + " ").collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("\n");
}

fn debug_grid(g: &Grid) -> String {
    return g.iter().enumerate().map(|(il, l)| l.iter().enumerate().map(|(ic, (v, ns))|
        if *v != 0 {
            format!("{:?} owned by {}: {:?}\n", Coord { x: ic as u8, y: il as u8 }, v, ns)
        } else {
            "".to_owned()
        }
    ).collect::<Vec<_>>().join("")).collect::<Vec<_>>().join("");
}

fn int_to_alpha(v: u8) -> char {
    ((v as u32) + ('A' as u32)) as u8 as char
}

fn get_index_peg_relative_to(_dest: &Coord, _from: &Coord) -> usize {
    let dy = _dest.y as i8 - _from.y as i8;
    let dx = _dest.x as i8 - _from.x as i8;
    match (dy, dx) {
        (-2, -1) => 0,
        (-2, 1) => 1,
        (-1, 2) => 2,
        (1, 2) => 3,
        (2, 1) => 4,
        (2, -1) => 5,
        (1, -2) => 6,
        (-1, -2) => 7,
        _ => panic!("Cannot find peg relative offset with offsets dy={} dx={}", dy, dx)
    }
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

        let mut grid = make_grid();

        parse_grid(&mut grid, 1);
        parse_segments(&mut grid, 1);
        parse_grid(&mut grid, 2);
        parse_segments(&mut grid, 2);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        eprintln!("grid is \n{}", show_grid(&grid));
        eprintln!("debug grid is \n{}", debug_grid(&grid));

        let a = random_pick(&mut rng, &grid, my_lines);

        eprintln!("{}{}", a.x, a.y);
        println!("{}{}", int_to_alpha(a.x), a.y + 1);
        turn += 1;
    }

    fn random_pick(rng: &mut ThreadRng, g: &Grid, my_lines: u8) -> Coord {
        loop {
            let x: u8 = rng.gen_range(0, GRID_SIZE as u8);
            let y: u8 = rng.gen_range(0, GRID_SIZE as u8);
            if g[y as usize][x as usize].0 == 0 {
                if my_lines == HORIZONTAL && (x == 0 || x == (GRID_SIZE - 1) as u8) {
                    continue;
                }
                if my_lines == VERTICAL && (y == 0 || y == (GRID_SIZE - 1) as u8) {
                    continue;
                }
                return Coord { x: x as u8, y: y as u8 };
            }
        }
    }

    fn play_action(mut g: Grid, c: Coord, p: u8) -> Grid {
        g[c.y as usize][c.x as usize].0 = p;
        g
    }
}

fn parse_segments(grid: &mut [[(u8, [bool; 8]); 12]; 12], num_player: u8) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num = parse_input!(input_line, i32);
    for _ in 0..num as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let p1 = parse_peg(inputs[0].trim().to_string());
        let p2 = parse_peg(inputs[1].trim().to_string());
        let index_peg = get_index_peg_relative_to(&p2, &p1);
        grid[p1.y as usize][p1.x as usize].0 = num_player;
        grid[p1.y as usize][p1.x as usize].1[index_peg] = true;
        let index_peg_rev = get_index_peg_relative_to(&p1, &p2);
        grid[p2.y as usize][p2.x as usize].0 = num_player;
        grid[p2.y as usize][p2.x as usize].1[index_peg_rev] = true;
    }
}

fn parse_grid(grid: &mut [[(u8, [bool; 8]); 12]; 12], num_player: u8) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num = parse_input!(input_line, i32);
    for _ in 0..num as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let p = parse_peg(input_line.trim().to_string());
        grid[p.y as usize][p.x as usize].0 = num_player;
    }
}
