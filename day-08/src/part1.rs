use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

pub fn update_screen(input: &str) -> usize {
    let mut screen = [[0_u8; WIDTH]; HEIGHT];
    let instructions = parse_instructions(input);
    for instruction in instructions {
        println!("\nInstruction: {:?}", instruction);
        match instruction {
            Instruction::Rect(w, h) => {
                for (x, y) in iproduct!(0..w, 0..h) {
                    screen[y][x] = 1;
                }
            }
            Instruction::RotateColumn(x, amount) => {
                let prev = (0..HEIGHT).map(|y| screen[y][x]).collect_vec();
                for y in 0..HEIGHT {
                    screen[y][x] =
                        prev[(y as i32 - amount as i32).rem_euclid(HEIGHT as i32) as usize]
                }
            }
            Instruction::RotateRow(y, amount) => {
                let prev = (0..WIDTH).map(|x| screen[y][x]).collect_vec();
                for x in 0..WIDTH {
                    screen[y][x] =
                        prev[(x as i32 - amount as i32).rem_euclid(WIDTH as i32) as usize]
                }
            }
        }

        // Well, this is part two.. lol.
        (0..HEIGHT).for_each(|y| {
            for x in 0..WIDTH {
                print!("{}", if screen[y][x] == 0 { '.' } else { '#' })
            }
            println!();
        });
    }
    screen.iter().flat_map(|row| row.iter()).sum::<u8>() as usize
}

#[derive(Debug)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new(r"(rect|rotate row|rotate column) (?:(\d+)x(\d+)|\w=(\d+) by (\d+))")
                    .unwrap();
        }

        let captures = INSTRUCTION_RE.captures(input).map(|captures| {
            captures
                .iter()
                .skip(1)
                .flatten()
                .map(|c| c.as_str())
                .collect_vec()
        });

        match captures.as_deref() {
            Some(["rect", width, height]) => {
                Self::Rect(width.parse().unwrap(), height.parse().unwrap())
            }
            Some(["rotate row", y, amount]) => {
                Self::RotateRow(y.parse().unwrap(), amount.parse().unwrap())
            }
            Some(["rotate column", x, amount]) => {
                Self::RotateColumn(x.parse().unwrap(), amount.parse().unwrap())
            }
            _ => panic!("Unknown instruction {}", input),
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.split('\n').map(Instruction::parse).collect_vec()
}
